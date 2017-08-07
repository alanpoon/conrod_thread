#[no_mangle]
pub  extern "C" fn get_static_styles()->Static_Style{
    Static_Style{
      w_h:(800.0,600.0),
      rect:(300.0,70.0,2.0),
    name:(40,RGB(0.82,0.27,0.25,1.0),100.0,50.0,5.0,5.0),
    image:(SpriteInfo{
      first:(0.0,270.0),
      num_in_row:4.0,
      w_h:(150.0,90.0),
      pad:(10.0,10.0,0.0,0.0)
    },30.0,30.0,5.0,5.0),
    text:(40,RGB(0.82,0.27,0.25,1.0),100.0,50.0,100.0,5.0),
  }
}
#[repr(C)]
pub struct SpriteInfo{
  first:(f64,f64), //left corner of first
  num_in_row:f64,
  w_h:(f64,f64),
  pad:(f64,f64,f64,f64),
}
#[repr(C)]
pub struct RGB(f32,f32,f32,f32);

pub struct Static_Style{
    pub w_h: (f64, f64),
    pub rect: (f64, f64,f64), //w,h, pad bottom
    pub name:(u32, RGB, f64, f64, f64, f64), // fontsize,RGB,w,h,l,t
    pub image: (SpriteInfo, f64, f64, f64, f64), // w,h,l,t
    pub text:(u32, RGB, f64, f64, f64, f64), // fontsize,RGB,w,h,l,t
}