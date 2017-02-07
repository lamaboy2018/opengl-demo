use gl::types::*;
use core::Type::Type;
use core::parse::glone::parseOne;


/*
match length = 6 parse ["[Tid=3652] ", " glGenBuffers", "GLsizei", "0x1", "P=", "0x33453"]
match length = 8 parse ["[Tid=3652] ", " glBindBuffer", "GLenum", "0x88EC", "P=", "0x33453", "P=", "0xBEE7453C"]
match length = 6 parse ["[Tid=3652] ", " glBufferData", "GLenum", "0x88EC", "GLsizeiptr", "0x120000"]
match length = 8 parse ["[Tid=3652] ", " glBindBuffer", "GLenum", "0x88EC", "P=", "0x0", "P=", "0xBEE7453C"]
match length = 6 parse ["[Tid=3652] ", " glGenTextures", "GLsizei", "0x1", "GLuint p=", "0x0"]
match length = 6 parse ["[Tid=3652] ", " glBindTexture", "GLenum", "0xDE1", "GLuint p=", "0x222E2"]
match length = 6 parse ["[Tid=3652] ", " glPixelStorei", "GLenum", "0xCF5", "GLint", "0x1"]

*/

pub fn parse2x(args: &mut [&str]){
}





