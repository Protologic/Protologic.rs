#[doc(hidden)]
#[macro_export]
macro_rules! protologic_define_extern
{
    ($vis:vis fn $name:ident ( $( $arg_name:ident : $arg_ty:ty ),* $(,)? ) $( -> $ret_ty:ty )?) =>
    {
        #[cfg(not(feature="mock_protologic"))]
        mod $name {
            #[link(wasm_import_module = "protologic")]
            extern {
                $vis fn $name ( $( $arg_name : $arg_ty ),* ) $( -> $ret_ty )?;
            }
        }

        #[allow(unused)]
        #[cfg(feature = "mock_protologic")]
        mod $name {
            $vis unsafe fn $name ( $( $arg_name : $arg_ty ),* ) $( -> $ret_ty )? { panic!("extern called in test mode!") }
        }

        $vis use self::$name::$name;
    }
}

pub mod lowlevel;
pub mod highlevel;
pub mod radio;
pub mod wait;

pub use highlevel::*;
