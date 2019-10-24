/// Used to define a property.
#[macro_export]
macro_rules! property {
    ($(#[$property_doc:meta])* $property:ident($type:ty) $(: $( $ex_type:ty ),*)* ) => {
        #[derive(Debug, Clone)]
        $(#[$property_doc])*
        pub struct $property(pub $type);

        impl $property {
            /// Returns the value of a property.
            pub fn get(key: &str, entity: Entity, store: &StringComponentStore) -> $type {
                get_property::<$property>(key, entity, store).0
            }

            /// Returns the value of a property if it exists otherwise the given value.
            pub fn get_or_value(key: &str, entity: Entity, store: &StringComponentStore, value: $property) -> $type {
                get_property_or_value::<$property>(key, entity, store, value).0
            }
        }

        impl IntoPropertySource<$property> for $type {
            fn into_source(self) -> PropertySource<$property> {
                PropertySource::Value(self.into())
            }
        }

         impl IntoPropertySource<$property> for $property {
            fn into_source(self) -> PropertySource<$property> {
                PropertySource::Value(self)
            }
        }

        $(
            $(
                impl IntoPropertySource<$property> for $ex_type {
                    fn into_source(self) -> PropertySource<$property> {
                        PropertySource::Value(self.into())
                    }
                }

                impl From<$ex_type> for $property {
                    fn from(value: $ex_type) -> $property {
                        $property(value.into())
                    }
                }
            )*
        )*

        impl IntoPropertySource<$property> for Entity {
            fn into_source(self) -> PropertySource<$property> {
                PropertySource::Source(self)
            }
        }

        impl From<$property> for $type {
            fn from(property: $property) -> $type {
                property.0.into()
            }
        }

        impl From<$type> for $property {
            fn from(value: $type) -> $property {
                $property(value)
            }
        }
    };
}

/// Used to define a widget, with properties and event handlers.
#[macro_export]
macro_rules! widget {
    ( $(#[$widget_doc:meta])* $widget:ident $(<$state:ident>)* $(: $( $handler:ident ),*)* $( { $($(#[$prop_doc:meta])* $property:ident: $property_type:tt ),* } )* ) => {
        $(#[$widget_doc])*
        #[derive(Default)]
        pub struct $widget {
            attached_properties: HashMap<String, ComponentBox>,
            shared_attached_properties: HashMap<String, SharedComponentBox>,
            event_handlers: Vec<Rc<dyn EventHandler>>,
            bounds: Bounds,
            position: Pos,
            min_width: Option<f64>,
            min_height: Option<f64>,
            max_width: Option<f64>,
            max_height: Option<f64>,
            width: Option<f64>,
            height: Option<f64>,
            name: Option<Name>,
            horizontal_alignment: HorizontalAlignment,
            vertical_alignment: VerticalAlignment,
            margin: Margin,
            enabled: Enabled,
            clip: Clip,
            visibility: Visibility,
            _empty: Option<RefCell<i32>>,
             $(
                $(
                    $property: Option<PropertySource<$property_type>>,
                )*
             )*
            children: Vec<Entity>,
            $(
                state: RefCell<Option<Rc<$state>>>
            )*
        }

        impl $widget {
            /// Sets or shares an attached property.
            pub fn attach<P: Component + Debug>(mut self, key: &str, property: impl IntoPropertySource<P>) -> Self {
                match property.into_source() {
                    PropertySource::Value(value) => {
                        self.attached_properties.insert(key.to_string(), ComponentBox::new(value));
                    },
                    PropertySource::Source(source) => {
                        self.shared_attached_properties.insert(key.to_string(), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                }
                self
            }

            /// Shares an attached property.
            pub fn attach_by_source<P: Component>(mut self, key: &str, source: Entity) -> Self {
                self.shared_attached_properties.insert(key.to_string(), SharedComponentBox::new(TypeId::of::<P>(), source));
                self
            }

            /// Sets or shares the constraint property.
            pub fn position(self, position: impl IntoPropertySource<Pos>) -> Self {
                self.attach("position", position)
            }

            /// Sets or shares the constraint property.
            pub fn constraint(self, constraint: impl IntoPropertySource<Constraint>) -> Self {
                self.attach("constraint", constraint)
            }

            /// Sets or shares the vertical alignment property.
            pub fn vertical_alignment(self, vertical_alignment: impl IntoPropertySource<VerticalAlignment>) -> Self {
                self.attach("vertical_alignment", vertical_alignment)
            }

            /// Sets or shares the horizontal alignment property.
            pub fn horizontal_alignment(self, horizontal_alignment: impl IntoPropertySource<HorizontalAlignment>) -> Self {
                self.attach("horizontal_alignment", horizontal_alignment)
            }

            /// Sets or shares the visibility property.
            pub fn visibility(self, visibility: impl IntoPropertySource<Visibility>) -> Self {
                self.attach("visibility", visibility)
            }

            /// Sets or shares the margin property.
            pub fn margin(self, margin: impl IntoPropertySource<Margin>) -> Self {
                self.attach("margin", margin)
            }

            /// Sets or shares the enabled property.
            pub fn enabled(self, enabled: impl IntoPropertySource<Enabled>) -> Self {
                self.attach("enabled", enabled)
            }

            /// Sets or shares the clip property.
            pub fn clip(self, clip: impl IntoPropertySource<Clip>) -> Self {
                self.attach("clip", clip)
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
            pub fn name<P: Into<Name>>(mut self, name: P) -> Self {
                self.name = Some(name.into());
                self
            }

            $(
                /// Returns the cloned state of the widget.
                pub fn clone_state(&self) -> Rc<$state> {
                    if let Some(state) = &*self.state.borrow() {
                        return state.clone();
                    }

                    let state = Rc::new($state::default());
                    *self.state.borrow_mut() = Some(state.clone());

                    state
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
        }

        $(
            $(
                impl $handler for $widget {}
            )*
        )*

        impl Widget for $widget {
            fn create() -> Self {
                $widget {
                    event_handlers: vec![],
                    enabled: Enabled(true),
                    clip: Clip(false),
                    $(
                        $(
                            $property: None,
                        )*
                    )*
                    children: vec![],
                    ..Default::default()
                }
            }

            fn insert_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
                self.event_handlers.push(handler.into());
                self
            }

            fn child(mut self, child: Entity) -> Self {
                self.children.push(child);
                self
            }

            $(
                fn state(&self) -> Option<Rc<State>> {
                    if self.state.borrow().is_none() {
                        *self.state.borrow_mut() = Some(Rc::new($state::default()));
                    }

                    if let Some(state) = &*self.state.borrow() {
                        return Some(state.clone())
                    }

                    None
                }
            )*

            fn build(self, context: &mut BuildContext) -> Entity {
                let entity = context.create_entity();

                let this = self.template(entity, context);

                context.register_render_object(entity, this.render_object());
                context.register_layout(entity, this.layout());

                 // register state
                if let Some(state) = &this.state() {
                     context.register_state(entity, state.clone());
                 }

                // register default set of properties
                context.register_property("bounds", entity, this.bounds);
                context.register_property("position", entity, this.position);
                context.register_property("vertical_alignment", entity, this.vertical_alignment);
                context.register_property("horizontal_alignment", entity, this.horizontal_alignment);
                context.register_property("visibility", entity, this.visibility);
                context.register_property("margin", entity, this.margin);
                context.register_property("enabled", entity, this.enabled);
                context.register_property("clip", entity, this.clip);

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
                context.register_property("constraint", entity, constraint);

                

                // register attached properties
                for (key, property) in this.attached_properties {
                    context.register_property_box(key.as_str(), entity, property);
                }

                for (key, property) in this.shared_attached_properties {
                    context.register_property_shared_box(key.as_str(), entity, property);
                }

                // register properties
                $(
                    $(
                        if let Some($property) = this.$property {
                            match $property {
                                PropertySource::Value(value) => {
                                    context.register_property(stringify!($property), entity, value);
                                },
                                PropertySource::Source(source) => {
                                    if stringify!($property) == "icon" {
                                        println!("Icon:");
                                    }
                                    context.register_shared_property::<$property_type>(stringify!($property), entity, source);
                                }
                            }
                        }
                        else {
                            context.register_property(stringify!($property), entity, $property_type::default());
                        }
                    )*
                )*

                // register event handlers
                for handler in this.event_handlers {
                    context.register_handler(entity, handler);
                }

                // register name
                if let Some(name) = this.name {
                    context.register_property("name", entity, name);
                }

                for child in this.children {
                    context.append_child(entity, child);
                }

                entity
            }
        }
    };
}
