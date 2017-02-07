use glx::GLfloat;
const   RED  : GLfloat = 0xFFFF0000 as GLfloat;
const  BLUE : GLfloat  = 0xFF0000FF as GLfloat;
const  YELLOW  : GLfloat     = 0xFFFFFF00 as GLfloat;
const  CYAN     : GLfloat     = 0xFF00FFFF as GLfloat ;
const GRAY      : GLfloat    = 0xFF888888 as GLfloat ;

pub fn red() ->[GLfloat;4]{
    [
        1.0,
        0.0,
        0.0,
        1.0
    ]
}


pub fn blue() ->[GLfloat;4]{
    [
        BLUE / 255.0,
        BLUE / 255.0,
        BLUE / 255.0,
        1.0
    ]
}

pub fn cyan() ->[GLfloat;4]{
    [
        CYAN / 255.0,
        CYAN / 255.0,
        CYAN / 255.0,
        1.0
    ]
}

pub fn gray() ->[GLfloat;4]{
    [
        GRAY / 255.0,
        GRAY / 255.0,
        GRAY / 255.0,
        1.0
    ]
}

pub fn yellow() ->[GLfloat;4]{
    [
        YELLOW / 255.0,
        YELLOW / 255.0,
        YELLOW / 255.0,
        1.0
    ]
}

