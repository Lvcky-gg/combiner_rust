fn get_nth_arg(n: usize)->
String {
    std::env::args().nth(n).unwrap()

}
//pub makes things usable outside of file
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}