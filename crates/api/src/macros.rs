#[macro_export]
macro_rules! into_property_source {
    ($type:ty $(: $( $ex_type:ty ),*)* ) => {
        impl IntoPropertySource<$type> for $type {
            fn into_source(self) -> PropertySource<$type> {
                PropertySource::Value(self)
            }
        }

        impl IntoPropertySource<$type> for Entity {
            fn into_source(self) -> PropertySource<$type> {
                PropertySource::Source(self)
            }
        }

        impl IntoPropertySource<$type> for (String, Entity) {
            fn into_source(self) -> PropertySource<$type> {
                PropertySource::KeySource(self.0, self.1)
            }
        }

        impl IntoPropertySource<$type> for (&str, Entity) {
            fn into_source(self) -> PropertySource<$type> {
                PropertySource::KeySource(self.0.to_string(), self.1)
            }
        }

         $(
            $(
                impl IntoPropertySource<$type> for $ex_type {
                    fn into_source(self) -> PropertySource<$type> {
                        PropertySource::Value(self.into())
                    }
                }
            )*
        )*
    };
}

/// Used to define a widget, with properties and event handlers.
#[macro_export]
macro_rules! widget {
    ( $(#[$widget_doc:meta])* $widget:ident $(<$state:ident>)* $(: $( $handler:ident ),*)*
            $( { $($(#[$prop_doc:meta])* $property:ident: $property_type:tt ),*
                $( attached_properties: { $($(#[$att_prop_doc:meta])* $att_property:ident: $att_property_type:tt ),* } )*
             } )* ) => {
        $(#[$widget_doc])*
        #[derive(Default, WidgetCtx)]
        #[allow(dead_code)]
        pub struct $widget {
            attached_properties: HashMap<String, ComponentBox>,
            shared_attached_properties: HashMap<(String, String), SharedComponentBox>,
            event_handlers: Vec<Rc<dyn EventHandler>>,
            #[property(Rectangle)]
            bounds: Rectangle,
            #[property(Point)]
            position: Point,
            #[property(Constraint)]
            constraint: Constraint,
            min_width: Option<f64>,
            min_height: Option<f64>,
            max_width: Option<f64>,
            max_height: Option<f64>,
            width: Option<f64>,
            height: Option<f64>,
            name: Option<String>,
            element: Option<String>,
            id: Option<String>,
            classes: HashSet<String>,
            #[property(Alignment)]
            h_align: Alignment,
            #[property(Alignment)]
            v_align: Alignment,
            #[property(Thickness)]
            margin: Thickness,
            #[property(bool)]
            enabled: bool,
            #[property(bool)]
            clip: bool,
            #[property(f32)]
            opacity: f32,
            #[property(Visibility)]
            visibility: Visibility,
            _empty: Option<RefCell<i32>>,
             $(
                $(
                    #[property($property_type)]
                    $property: Option<PropertySource<$property_type>>,
                )*
             )*
            children: Vec<Entity>,
            $(
                state: Box<$state>
            )*
        }

        impl $widget {
            // internal helper
            fn set_property<P: Component + Debug>(mut self, key: &str, property: impl IntoPropertySource<P>) -> Self {
                match property.into_source() {
                    PropertySource::Value(value) => {
                        self.attached_properties.insert(key.to_string(), ComponentBox::new(value));
                    },
                    PropertySource::Source(source) => {
                        self.shared_attached_properties.insert((key.to_string(), key.to_string()), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                    PropertySource::KeySource(source_key, source) => {
                        self.shared_attached_properties.insert((key.to_string(), source_key), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                }
                self
            }

            /// Sets the id selector.
            pub fn id(mut self, id: impl Into<String>) -> Self {
                if !self.id.is_none() {
                    return self;
                }
                self.id = Some(id.into());
                self
            }

            /// Sets the element selector.
            pub fn element(mut self, element: impl Into<String>) -> Self {
                if !self.element.is_none() {
                    return self;
                }

                self.element = Some(element.into());
                self
            }

            /// Inserts class selector.
            pub fn class(mut self, class: impl Into<String>) -> Self {
                self.classes.insert(class.into());
                self
            }

            /// Sets or shares the position of the widget. (Be careful the position could be adjusted by layouts).
            pub fn position(self, position: impl IntoPropertySource<Point>) -> Self {
                self.set_property("position", position)
            }

            /// Sets or shares the constraint property.
            pub fn constraint(self, constraint: impl IntoPropertySource<Constraint>) -> Self {
                self.set_property("constraint", constraint)
            }

            /// Sets or shares the vertical alignment property.
            #[inline(always)]
            pub fn v_align(self, v_align: impl IntoPropertySource<Alignment>) -> Self {
                self.set_property("v_align", v_align)
            }

             /// Sets or shares the horizontal alignment property.
             #[inline(always)]
             pub fn h_align(self, h_align: impl IntoPropertySource<Alignment>) -> Self {
                 self.set_property("h_align", h_align)
             }

            /// Sets or shares the vertical alignment property.
            #[inline(always)]
            #[deprecated = "Use v_align instead"]
            pub fn vertical_alignment(self, vertical_alignment: impl IntoPropertySource<Alignment>) -> Self {
                self.v_align(vertical_alignment)
            }

            /// Sets or shares the horizontal alignment property.
            #[inline(always)]
            #[deprecated = "Use h_align instead"]
            pub fn horizontal_alignment(self, horizontal_alignment: impl IntoPropertySource<Alignment>) -> Self {
                self.h_align(horizontal_alignment)
            }

            /// Sets or shares the visibility property.
            pub fn visibility(self, visibility: impl IntoPropertySource<Visibility>) -> Self {
                self.set_property("visibility", visibility)
            }

            /// Sets or shares the margin property.
            pub fn margin(self, margin: impl IntoPropertySource<Thickness>) -> Self {
                self.set_property("margin", margin)
            }

            /// Sets or shares the enabled property.
            pub fn enabled(self, enabled: impl IntoPropertySource<bool>) -> Self {
                self.set_property("enabled", enabled)
            }

            /// Sets or shares the clip property.
            pub fn clip(self, clip: impl IntoPropertySource<bool>) -> Self {
                self.set_property("clip", clip)
            }

            // Sets or shares the opacity property.
            pub fn opacity(self, opacity: impl IntoPropertySource<f32>) -> Self {
                self.set_property("opacity", opacity)
            }

            /// Inserts a new width.
            pub fn width(mut self, width: f64) -> Self {
                if !self.width.is_none() {
                    return self;
                }
                self.width = Some(width);
                self
            }

            /// Inserts a new height.
            pub fn height(mut self, height: f64) -> Self {
                if !self.height.is_none() {
                    return self;
                }
                self.height = Some(height);
                self
            }

            /// Inserts a new size.
            pub fn size(mut self, width: f64, height: f64) -> Self {
                if self.width.is_none() {
                    self.width = Some(width);
                }
                if self.height.is_none() {
                    self.height = Some(height);
                }
                self
            }

            /// Inserts a new min_width.
            pub fn min_width(mut self, min_width: f64) -> Self {
                if !self.min_width.is_none() {
                    return self;
                }
                self.min_width = Some(min_width);
                self
            }

            /// Inserts a new min_height.
            pub fn min_height(mut self, min_height: f64) -> Self {
                if !self.min_height.is_none() {
                    return self;
                }
                self.min_height = Some(min_height);
                self
            }

            /// Inserts a new min_size.
            pub fn min_size(mut self, min_width: f64, min_height: f64) -> Self {
                if self.min_width.is_none() {
                    self.min_width = Some(min_width);
                }
                if self.min_height.is_none() {
                    self.min_height = Some(min_height);
                }
                self
            }

            /// Inserts a new max_width.
            pub fn max_width(mut self, max_width: f64) -> Self {
                if !self.max_width.is_none() {
                    return self;
                }
                self.max_width = Some(max_width);
                self
            }

            /// Inserts a new max_height.
            pub fn max_height(mut self, max_height: f64) -> Self {
                if !self.max_height.is_none() {
                    return self;
                }
                self.max_height = Some(max_height);
                self
            }

            /// Inserts a new min_size.
            pub fn max_size(mut self, max_width: f64, max_height: f64) -> Self {
                if self.max_width.is_none() {
                    self.max_width = Some(max_width);
                }
                if self.max_height.is_none() {
                    self.max_height = Some(max_height);
                }
                self
            }

            /// Sets the debug name of the widget.
            pub fn name<P: Into<String>>(mut self, name: P) -> Self {
                self.name = Some(name.into());
                self
            }

            $(
                /// Gets a reference of the state.
                fn state(&self) -> &Box<$state> {
                    &self.state
                }

                /// Gets a mutable reference of the state.
                fn state_mut(&mut self) -> &mut Box<$state> {
                    &mut self.state
                }
            )*

            $(
                $(
                    $(#[$prop_doc])*
                    pub fn $property<P: IntoPropertySource<$property_type>>(mut self, $property: P) -> Self {
                        if !self.$property.is_none() {
                            return self;
                        }

                        self.$property = Some($property.into_source());
                        self
                    }
                )*
            )*

            $(
                $(
                    $(
                        $(#[$att_prop_doc])*
                        pub fn $att_property(property: impl IntoPropertySource<$att_property_type>) -> AttachedProperty<$att_property_type> {
                            AttachedProperty::new(stringify!($att_property), property)
                        }
                    )*
                )*
            )*
        }

        $(
            $(
                impl $handler for $widget {}
            )*
        )*

        impl Widget for $widget {
            /// Creates a new widget.
            #[inline]
            fn new() -> Self {
                $widget {
                    event_handlers: vec![],
                    enabled: true,
                    opacity: 1.,
                    clip: false,
                    $(
                        $(
                            $property: None,
                        )*
                    )*
                    children: vec![],
                    ..Default::default()
                }
            }

            fn attach<P: Component + Debug>(mut self, property: AttachedProperty<P>) -> Self {
                match property.property_source {
                    PropertySource::Value(value) => {
                        self.attached_properties.insert(property.key, ComponentBox::new(value));
                    }
                    PropertySource::Source(source) => {
                        self.shared_attached_properties.insert((property.key.clone(), property.key), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                    PropertySource::KeySource(source_key, source) => {
                        self.shared_attached_properties.insert((property.key, source_key), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                }
                self
            }

            fn insert_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
                self.event_handlers.push(handler.into());
                self
            }

            fn child(mut self, child: Entity) -> Self {
                self.children.push(child);
                self
            }

            fn build(self, ctx: &mut BuildContext) -> Entity {
                let entity = ctx.create_entity();

                let this = self.template(entity, ctx);

                ctx.register_render_object(entity, this.render_object());
                ctx.register_layout(entity, this.layout());

                $(
                    // workaround
                    let _ = TypeId::of::<$state>();
                    ctx.register_state(entity, this.state);
                )*

                // register default set of properties
                ctx.register_property("bounds", entity, this.bounds);
                ctx.register_property("position", entity, this.position);
                ctx.register_property("v_align", entity, this.v_align);
                ctx.register_property("h_align", entity, this.h_align);
                ctx.register_property("visibility", entity, this.visibility);
                ctx.register_property("margin", entity, this.margin);
                ctx.register_property("enabled", entity, this.enabled);
                ctx.register_property("clip", entity, this.clip);
                ctx.register_property("opacity", entity, this.opacity);
                ctx.register_property("type_id", entity, TypeId::of::<$widget>());
                ctx.register_property("type_name", entity, std::any::type_name::<$widget>().to_string());

                if this.element.is_some() || this.id.is_some() || this.classes.len() > 0 {
                    let mut selector = Selector::new();
                    //selector.set_dirty(true);
                    if let Some(element) = this.element {
                        selector = selector.with(element);
                    }
                    if let Some(id) = this.id {
                        selector = selector.id(id);
                    }
                    let mut classes = this.classes;
                    for class in classes.drain() {
                        selector = selector.class(class);
                    }
                    ctx.register_property("selector", entity, selector);
                }


                let mut constraint = Constraint::default();

                if let Some(width) = this.width {
                    constraint.set_width(width);
                }
                if let Some(height) = this.height {
                    constraint.set_height(height);
                }
                if let Some(min_width) = this.min_width {
                    constraint.set_min_width(min_width);
                }
                if let Some(min_height) = this.min_height {
                    constraint.set_min_height(min_height);
                }
                if let Some(max_width) = this.max_width {
                    constraint.set_max_width(max_width);
                }
                if let Some(max_height) = this.max_height {
                    constraint.set_max_height(max_height);
                }
                ctx.register_property("constraint", entity, constraint);


                // register attached properties
                for (key, property) in this.attached_properties {
                    ctx.register_property_box(key.as_str(), entity, property);
                }

                for (key, property) in this.shared_attached_properties {
                    ctx.register_property_shared_box_by_source_key(key.0.as_str(), key.1.as_str(), entity, property);
                }

                // register properties
                $(
                    $(
                        if let Some($property) = this.$property {
                            match $property {
                                PropertySource::Value(value) => {
                                    ctx.register_property(stringify!($property), entity, value);
                                }
                                PropertySource::Source(source) => {
                                    ctx.register_shared_property::<$property_type>(stringify!($property), entity, source);
                                }
                                PropertySource::KeySource(source_key, source) => {
                                    ctx.register_shared_property_by_source_key::<$property_type>(stringify!($property), source_key.as_str(), entity, source);
                                }
                            }
                        }
                        else {
                            ctx.register_property(stringify!($property), entity, $property_type::default());
                        }
                    )*
                )*

                ctx.update_theme_by_state(entity);

                // register event handlers
                for handler in this.event_handlers {
                    ctx.register_handler(entity, handler);
                }

                // register name
                if let Some(name) = this.name {
                    ctx.register_property("name", entity, name);
                }

                for child in this.children {
                    ctx.append_child(entity, child);
                }

                entity
            }
        }
    };
}
