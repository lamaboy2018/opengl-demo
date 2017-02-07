
use cgmath::Vector3;
use cgmath::Point3;
use cgmath::Matrix4;
use cgmath::Rad;

pub type Matrix4x4 = [[f32;4];4];

pub struct camera{
    pos:Matrix4<f32>,

}

impl camera{
    pub fn setindety(&mut self){
        for i in 0..3 {
            for j in 0..3 {
                self.pos[i][j] =0.0;
            }
        }

    }

   pub fn  lookAt(eyePos:Point3<f32> ,  target:Point3<f32>,  up :Vector3<f32>)->Matrix4<f32>{
     Matrix4::look_at(eyePos,target,up)
    }

    pub fn roatoz(x:f32,y:f32,z:f32)->Matrix4<f32>{

        let mut matx :Matrix4<f32> ;
        let mut maty :Matrix4<f32> ;
        let mut matz :Matrix4<f32> ;

         matx =  Matrix4::from_angle_x(Rad(x));
         maty =  Matrix4::from_angle_y(Rad(y));
         matz =  Matrix4::from_angle_z(Rad(z));

        matx * maty * matz
    }

 /*   pub fn Mat2vec(mat:Matrix4<f32>) -> [f32;16]
    {

    }*/

}


