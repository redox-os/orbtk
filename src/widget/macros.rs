// This is only a early concept of the template macro!!!

// use std::rc::Rc;
#[macro_export]
macro_rules! impl_template {
    (($t:ident ( $v:ident ) )) => {{
        println!("Test");
        let mut blub: Rc<Widget> = Rc::new($v {
            ..Default::default()
        });
        $t(blub.clone())
    }};

    (($t:ident::new( $v:expr ) )) => {
       {
           println!("Exp");
            $t::new($v)
       }
    };

     (($t:ident::new( $v:ty ) )) => {
        $t::new($v::default())
    };

    ($t:expr) => {
        println!("expr");
    };

    ($t:ident) => {
        println!("ident");
    };

    ($t:pat) => {
        println!("pat");
    };

    ($t:tt) => {
        println!("tt");
    }; //   ( $t:ident($v:ident) ) => {
       //         {
       //             println!("None");
       //         None
       //         }
       //     };

       //     // ($t:ident) => {
       //     //     {
       //     //         Some(Rc::new($t {
       //     //             ..Default::default()
       //     //         }))
       //     //     }
       //     // };

       //     ($w:tt($t:ident)) => {
       //           {
       //             println!("None b");
       //         None
       //        }
       //     };

       //     ($other:tt) => {
       //        {
       //             println!("None a");
       //         None
       //        }
       //     };

       //     ($t:ident) => {
       //         {
       //             // Some(Rc::new($t {
       //             //     ..Default::default()
       //             // }))
       //              println!("None c");
       //         None
       //         }
       //     };

       //     // ($t:ident) => {
       //     //     Rc::new($t::default())
       //     // };
}

#[macro_export]
macro_rules! template {



    ($widget:ident { $($key:tt : $value:tt),* } ) => {
         {
             let mut widget = Rc::new($widget {
                $(
                    $key: impl_template!($value),
                )*
                ..Default::default()
             });




             widget
            // let mut widget = Rc::new($widget {
            //     $($key: impl_template!($value))*,
            //     ..Default::default()
            // });
            // widget
        }
    };



    // ( $( $t:ident { $( $key:tt : $value:tt ),* } ),* ) => {
    //      fn template(&self) -> Template {
    //         Template::Mutli(vec![$(impl_template!($t { $( $key, $value)* }))*])
    //     }
    // };
}
