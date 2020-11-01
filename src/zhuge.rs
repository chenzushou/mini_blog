//猪哥类
struct Zhuge{
    point:u8,
}
impl Zhuge {
    fn new()->Zhuge{
        Zhuge{
            point:10,
        }
    }
    fn unsatisfactory(&mut  self){
        if self.point==0 {
            return;
        }else{
            self.point=self.point-2;
        }
    }
}


