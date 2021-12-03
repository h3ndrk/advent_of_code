fn main() {
    let numbers = vec![
        0b000001100010,
        0b100111011010,
        0b001100011001,
        0b011010001010,
        0b011010101011,
        0b001001110101,
        0b100110001101,
        0b100010010011,
        0b011110000110,
        0b011000110110,
        0b011111111110,
        0b100111110011,
        0b110000100101,
        0b011010011111,
        0b001010011101,
        0b110110001010,
        0b101111110101,
        0b000010101101,
        0b111010111110,
        0b111110001110,
        0b100110111011,
        0b101011010101,
        0b001001000101,
        0b111101000100,
        0b001100000010,
        0b010001100101,
        0b001010011111,
        0b101010010100,
        0b100001110011,
        0b100000111010,
        0b100101010101,
        0b000000101000,
        0b011010100011,
        0b110100111110,
        0b010010011000,
        0b000101000010,
        0b001100011100,
        0b011101000000,
        0b010110111010,
        0b101101100100,
        0b000000010011,
        0b000101001111,
        0b011011100111,
        0b101111100101,
        0b101001101011,
        0b101100010100,
        0b001001110111,
        0b110011011110,
        0b111001101000,
        0b101101111111,
        0b001100110100,
        0b000001000111,
        0b000111101111,
        0b110101011100,
        0b010111011100,
        0b010011101000,
        0b100011101011,
        0b110000110011,
        0b110000100010,
        0b100101001110,
        0b011000001100,
        0b100000101110,
        0b101010110001,
        0b000111101110,
        0b011111101111,
        0b000000100010,
        0b101000010100,
        0b011000111001,
        0b001011001101,
        0b011111111111,
        0b100111000100,
        0b101010011010,
        0b001100100011,
        0b011101011110,
        0b011111100010,
        0b110111110011,
        0b111011111010,
        0b000101101011,
        0b001111011011,
        0b000011101000,
        0b000101111011,
        0b000010100110,
        0b111001000100,
        0b011110001111,
        0b010001011000,
        0b100010100100,
        0b100100111100,
        0b010010010101,
        0b001101110011,
        0b000000010000,
        0b000110001100,
        0b100110110111,
        0b001000100100,
        0b101010111010,
        0b000011101100,
        0b100000100001,
        0b011111110000,
        0b101001001111,
        0b100000111111,
        0b110111000111,
        0b111000010001,
        0b011011011111,
        0b110011011010,
        0b100011010101,
        0b011110010100,
        0b001001101010,
        0b100100111000,
        0b100110110011,
        0b010010111000,
        0b111011101111,
        0b111001101010,
        0b100000010011,
        0b011011101111,
        0b011100101011,
        0b110101011000,
        0b101111011110,
        0b100111000010,
        0b001110110101,
        0b111001111100,
        0b010000110111,
        0b011100001010,
        0b110111110101,
        0b011011100010,
        0b111101101000,
        0b101000000111,
        0b110000001001,
        0b001100101101,
        0b100111100010,
        0b000100010100,
        0b111101100110,
        0b101001010001,
        0b011000000001,
        0b010100100100,
        0b010001001000,
        0b010000010111,
        0b100011110001,
        0b000011100110,
        0b001001000001,
        0b000100001100,
        0b001001101111,
        0b001001100000,
        0b000111011100,
        0b100001010110,
        0b101010001110,
        0b010010101001,
        0b110010001100,
        0b101011101100,
        0b111000110111,
        0b100011010100,
        0b001110001101,
        0b101011011010,
        0b110101011010,
        0b011100010001,
        0b010011010011,
        0b001011111001,
        0b100011101100,
        0b011101100011,
        0b010000011111,
        0b011001101011,
        0b011110011100,
        0b001100011010,
        0b101100001110,
        0b011001111101,
        0b100101011111,
        0b011111100001,
        0b111001010111,
        0b010111000100,
        0b111110110111,
        0b100011011001,
        0b100000010000,
        0b111110111111,
        0b101111101011,
        0b110011010011,
        0b100001111110,
        0b101011011011,
        0b110111111110,
        0b101110101001,
        0b011100001100,
        0b000101011100,
        0b011010100111,
        0b110111011000,
        0b100110111100,
        0b111010001100,
        0b010010011111,
        0b000011010100,
        0b010111101111,
        0b111111000011,
        0b000111111010,
        0b000011111111,
        0b010010001010,
        0b100101110010,
        0b111110010100,
        0b111110000000,
        0b111110111110,
        0b110010100111,
        0b110011000010,
        0b001101010011,
        0b100100010001,
        0b101010110110,
        0b110110011100,
        0b111111001011,
        0b000011010001,
        0b010111011001,
        0b010100100010,
        0b010001101100,
        0b001101001110,
        0b010101000100,
        0b111010010010,
        0b000100100101,
        0b000011001001,
        0b011000110111,
        0b010111110111,
        0b011000100110,
        0b011110001110,
        0b011100111010,
        0b001111101100,
        0b100100001100,
        0b001100110101,
        0b011101111001,
        0b100001000000,
        0b110100001010,
        0b110111111000,
        0b010110010001,
        0b110010011101,
        0b101101011111,
        0b111111100110,
        0b101110001000,
        0b111100110100,
        0b110001111011,
        0b001011000011,
        0b011000000100,
        0b101100000001,
        0b001110000100,
        0b111110001111,
        0b100001001101,
        0b111111111011,
        0b101101110001,
        0b111001110111,
        0b100001101000,
        0b001100101001,
        0b000101000000,
        0b001011111011,
        0b000001011111,
        0b100000101011,
        0b101100000000,
        0b101110001001,
        0b110111111010,
        0b010000011000,
        0b110111100100,
        0b001100010011,
        0b111111100100,
        0b101011010100,
        0b101101001100,
        0b110110101001,
        0b011001101000,
        0b011101110001,
        0b010010010000,
        0b110001010101,
        0b110111110001,
        0b100011111110,
        0b110011111101,
        0b011111110110,
        0b110110101101,
        0b000010111000,
        0b111110011000,
        0b100111101110,
        0b101011000010,
        0b111101111000,
        0b001001110010,
        0b011000100101,
        0b011010100100,
        0b101111010001,
        0b111101101011,
        0b101110100110,
        0b011111100000,
        0b000101100001,
        0b110100100000,
        0b010011110111,
        0b001011010011,
        0b010001101111,
        0b100111110010,
        0b100111010001,
        0b110001111111,
        0b000110110011,
        0b000100000101,
        0b011111111001,
        0b110011100110,
        0b111011000110,
        0b100101101101,
        0b001111000010,
        0b001101011100,
        0b011001101111,
        0b101110111100,
        0b110011001110,
        0b011001011000,
        0b110010001001,
        0b000100000110,
        0b011101001000,
        0b100111010111,
        0b000101010000,
        0b011110110110,
        0b010110111001,
        0b111110100101,
        0b100100011010,
        0b111011100110,
        0b100111010000,
        0b011111100111,
        0b101010100001,
        0b110001101110,
        0b000101000001,
        0b010011011100,
        0b111101011111,
        0b101001100111,
        0b011101111000,
        0b100001100111,
        0b110011001001,
        0b011101000111,
        0b000011011100,
        0b100111001001,
        0b001001101100,
        0b101101111101,
        0b100100001111,
        0b111100111011,
        0b010011000001,
        0b000101010101,
        0b000100100010,
        0b100001011100,
        0b110011100010,
        0b010110011010,
        0b001110100001,
        0b001110111011,
        0b101110100111,
        0b011101101110,
        0b110100101110,
        0b111101110010,
        0b001001110110,
        0b100011010111,
        0b111110101100,
        0b011100111101,
        0b001010000100,
        0b000111001111,
        0b110101010110,
        0b101010101011,
        0b011101011000,
        0b111000111010,
        0b000010011001,
        0b011010111101,
        0b000111111101,
        0b000010111101,
        0b111011000000,
        0b111100001100,
        0b110101111111,
        0b000111011000,
        0b010010001111,
        0b100011101000,
        0b110100010001,
        0b110101001111,
        0b100011100100,
        0b000010111100,
        0b011010010011,
        0b111001000010,
        0b101010101111,
        0b110100101010,
        0b001100101011,
        0b001111101011,
        0b000011100111,
        0b111011001001,
        0b110110000010,
        0b010000010011,
        0b010110110010,
        0b110100100101,
        0b001101110101,
        0b001011001011,
        0b010100100000,
        0b000010000000,
        0b110110011101,
        0b101001100011,
        0b010111000001,
        0b001111100111,
        0b110111111101,
        0b011010100000,
        0b011011110101,
        0b011110100001,
        0b010100110001,
        0b100000000101,
        0b010011101111,
        0b101111110111,
        0b101000100100,
        0b011010101001,
        0b010000000110,
        0b110011110111,
        0b010011011011,
        0b101101101111,
        0b001111011101,
        0b100111101111,
        0b000111000101,
        0b100100101010,
        0b011110011110,
        0b010011101110,
        0b000000100110,
        0b010100110101,
        0b101101010100,
        0b101001000010,
        0b111000000110,
        0b010001000000,
        0b000100010110,
        0b110101101001,
        0b010010111110,
        0b010011100001,
        0b001011111111,
        0b111001101001,
        0b111100001110,
        0b101110010101,
        0b111101111011,
        0b110011010000,
        0b111101011011,
        0b111100101101,
        0b111010100110,
        0b111000001011,
        0b011011001011,
        0b000000010010,
        0b100100000000,
        0b110011111001,
        0b010000101000,
        0b111010110100,
        0b010101101011,
        0b111111001111,
        0b010101001110,
        0b101110111011,
        0b011101111010,
        0b101110110011,
        0b000110111101,
        0b111100011100,
        0b000010101001,
        0b000011011010,
        0b100010011100,
        0b000110100001,
        0b101001111110,
        0b000010100010,
        0b001110000000,
        0b100011100110,
        0b010011110110,
        0b110000111000,
        0b010110001110,
        0b110001100011,
        0b001010000011,
        0b101001010000,
        0b101110110100,
        0b111010000100,
        0b111011011001,
        0b000100001011,
        0b011010001101,
        0b011000101000,
        0b011101011100,
        0b110111110100,
        0b101000000010,
        0b101000100011,
        0b011101110011,
        0b011100011010,
        0b001010001111,
        0b001011101011,
        0b100001010010,
        0b111110111101,
        0b110000010001,
        0b011001010101,
        0b100100101000,
        0b110000000111,
        0b001111110101,
        0b101000010010,
        0b011110111100,
        0b101111111000,
        0b000011101101,
        0b110001100110,
        0b001111011000,
        0b000011000100,
        0b001101000010,
        0b011111111000,
        0b011111001010,
        0b000110000010,
        0b010000010110,
        0b101111000010,
        0b001101111101,
        0b101011101010,
        0b101110110010,
        0b001000000111,
        0b100000101001,
        0b011111110010,
        0b011001100100,
        0b011010010111,
        0b001010110011,
        0b001001011010,
        0b000111000001,
        0b000111000000,
        0b011000001111,
        0b010001001101,
        0b001100010110,
        0b010100000011,
        0b110110110100,
        0b011100000100,
        0b101111000101,
        0b110101010010,
        0b001110010111,
        0b110000011010,
        0b000110100000,
        0b111101000011,
        0b000100011100,
        0b010010100010,
        0b000000010001,
        0b110001111101,
        0b100011001101,
        0b010011110011,
        0b000101010001,
        0b110111101001,
        0b110010000011,
        0b100100000101,
        0b100001010101,
        0b111111111110,
        0b100011101101,
        0b100100100011,
        0b110000001111,
        0b010101001010,
        0b101000111000,
        0b010010011100,
        0b110011101001,
        0b011101011111,
        0b011111001011,
        0b111110011100,
        0b010001111111,
        0b100011110100,
        0b010000001101,
        0b100111111100,
        0b101000101011,
        0b110110101011,
        0b110101001100,
        0b101010110010,
        0b110110100011,
        0b001001010001,
        0b011110010000,
        0b000001111111,
        0b101010101000,
        0b010001111101,
        0b100110101111,
        0b100101000011,
        0b001010110010,
        0b001111001100,
        0b100101011100,
        0b111001000101,
        0b010100100101,
        0b100010101101,
        0b100101110001,
        0b101000100010,
        0b110101111110,
        0b111100110110,
        0b100101100100,
        0b001000001100,
        0b010011110001,
        0b011100101000,
        0b101111101010,
        0b100101100001,
        0b000110000000,
        0b001001001111,
        0b001111100000,
        0b111110001011,
        0b011110011010,
        0b111001100110,
        0b000010100000,
        0b010110110001,
        0b100111001101,
        0b111101010110,
        0b110001010100,
        0b110110100000,
        0b011100000011,
        0b100101110111,
        0b000001000011,
        0b100100100110,
        0b111100000010,
        0b100110110100,
        0b000101111110,
        0b001110100110,
        0b010101110100,
        0b111010110101,
        0b000001100111,
        0b100010001101,
        0b101011011111,
        0b010011111101,
        0b101010011111,
        0b000011111000,
        0b110001100000,
        0b100010111011,
        0b010111101011,
        0b010100101011,
        0b100010001110,
        0b111100000100,
        0b101011101110,
        0b000101100011,
        0b100000111000,
        0b110011001010,
        0b000001110011,
        0b000111010100,
        0b111110110100,
        0b100101011001,
        0b110010010001,
        0b010100111101,
        0b111100100000,
        0b101000110000,
        0b011101111111,
        0b001101111111,
        0b001001000110,
        0b000001100110,
        0b110100101000,
        0b001111000000,
        0b111101100010,
        0b100111001011,
        0b001001100101,
        0b100000001001,
        0b101010100101,
        0b010001000010,
        0b111100000000,
        0b001100010101,
        0b101111111101,
        0b110100010100,
        0b101000100101,
        0b111001010000,
        0b011001000100,
        0b011110011011,
        0b101100010011,
        0b100100110111,
        0b111100101011,
        0b011100100110,
        0b000111001001,
        0b111101101001,
        0b001111101111,
        0b101111111001,
        0b101100110101,
        0b101000011000,
        0b100100011111,
        0b101011001011,
        0b001101111001,
        0b111010011100,
        0b001011000001,
        0b100011001110,
        0b101101010010,
        0b111000101010,
        0b101101100110,
        0b101111010100,
        0b101011000100,
        0b100111111010,
        0b110110001011,
        0b100101001001,
        0b110001101100,
        0b010000100000,
        0b110100010010,
        0b101011110001,
        0b001000011010,
        0b001011110000,
        0b101111010110,
        0b000101110111,
        0b110111110000,
        0b101110001100,
        0b111010110001,
        0b010001110001,
        0b100010001000,
        0b001001000111,
        0b100010110010,
        0b000011100101,
        0b111111000111,
        0b010010001110,
        0b011011011110,
        0b111110100011,
        0b101111111111,
        0b000010011111,
        0b010101011101,
        0b101101011011,
        0b001111010111,
        0b001010010100,
        0b101010111001,
        0b010110111111,
        0b000110111110,
        0b001101101100,
        0b011000101001,
        0b110011000001,
        0b100111001010,
        0b001101011010,
        0b100000100111,
        0b110100010111,
        0b100000111001,
        0b100011011111,
        0b010111100011,
        0b110111000010,
        0b011110101001,
        0b010110101000,
        0b011110111110,
        0b110000110001,
        0b100100001010,
        0b011111001000,
        0b110110110010,
        0b011000101011,
        0b000000000010,
        0b000010000100,
        0b110000000011,
        0b011111101101,
        0b011000101101,
        0b110011101110,
        0b000000001001,
        0b101001110010,
        0b101101000001,
        0b011011001111,
        0b110010010010,
        0b000100101011,
        0b010111010001,
        0b111101000110,
        0b111010111111,
        0b111001010100,
        0b011010110101,
        0b001001000010,
        0b011001000010,
        0b000000101101,
        0b010000111110,
        0b010011100101,
        0b100111000110,
        0b111111001100,
        0b101100100110,
        0b011110100110,
        0b001110101101,
        0b001101001000,
        0b111111011000,
        0b011100100000,
        0b001000011001,
        0b001001111111,
        0b111010011011,
        0b001001111101,
        0b101000110110,
        0b100000111101,
        0b001011000110,
        0b100010010010,
        0b101010011001,
        0b101010110011,
        0b101010110000,
        0b000001000110,
        0b001101110110,
        0b010010011011,
        0b101111110000,
        0b011110001011,
        0b001011111100,
        0b111011111111,
        0b101101010111,
        0b000001010101,
        0b010110101111,
        0b111100000001,
        0b010101100010,
        0b101111010011,
        0b010001011010,
        0b011100101101,
        0b111100101010,
        0b001100111001,
        0b001101000111,
        0b011010111010,
        0b010011100000,
        0b110000011001,
        0b111101100101,
        0b010111101110,
        0b011110001010,
        0b011101000100,
        0b011000110011,
        0b110111001000,
        0b011000001001,
        0b010111111001,
        0b111001110010,
        0b110110110000,
        0b000100110000,
        0b101011110110,
        0b111000011010,
        0b011001110100,
        0b100011101001,
        0b111010110110,
        0b000110100010,
        0b011101010110,
        0b001100111010,
        0b010010101101,
        0b000101110011,
        0b100000000010,
        0b100111100011,
        0b101110111000,
        0b100001111011,
        0b001111001000,
        0b000110010110,
        0b010001110101,
        0b111100010011,
        0b100000010101,
        0b111111111101,
        0b000001001100,
        0b100111111110,
        0b111100110001,
        0b111000100011,
        0b110101100110,
        0b000101010100,
        0b010000001001,
        0b011010000011,
        0b011011011000,
        0b101100011010,
        0b101001001011,
        0b000010110111,
        0b101111000111,
        0b100101010001,
        0b001111011001,
        0b000010111111,
        0b010001010100,
        0b011110101111,
        0b001110101010,
        0b010000001100,
        0b011011101011,
        0b100111110100,
        0b101101100101,
        0b101101000011,
        0b111110101111,
        0b110010110110,
        0b100011000101,
        0b011011110011,
        0b101010101100,
        0b001101000001,
        0b100010111010,
        0b100101011101,
        0b101011000001,
        0b101100000011,
        0b001011000100,
        0b000011100000,
        0b010110001011,
        0b000111111100,
        0b111011000011,
        0b110100111001,
        0b001110001110,
        0b001010100001,
        0b110111111001,
        0b011011101110,
        0b010100010000,
        0b101011111000,
        0b010001100001,
        0b000001010111,
        0b110011101010,
        0b001110111110,
        0b011101110110,
        0b010110001000,
        0b110101111010,
        0b100101100000,
        0b000100111001,
        0b110110111010,
        0b111010010101,
        0b110111111100,
        0b010101010111,
        0b100001011110,
        0b010101001001,
        0b001101011101,
        0b011100100100,
        0b010001110111,
        0b100111001100,
        0b101010111011,
        0b100000111110,
        0b101111011011,
        0b011110110001,
        0b000100000100,
        0b010111100100,
        0b110101010111,
        0b101101101011,
        0b110101101010,
        0b001010000111,
        0b111001111111,
        0b100100011001,
        0b101111010000,
        0b110111011010,
        0b110001110101,
        0b011100001001,
        0b001111100011,
        0b000100010111,
        0b100110010011,
        0b011110011001,
        0b010110001111,
        0b011111011010,
        0b111011111000,
        0b011110111101,
        0b000001000100,
        0b111000010000,
        0b010010000101,
        0b011100100111,
        0b110001001000,
        0b110100111111,
        0b100000010110,
        0b110111100011,
        0b101001110100,
        0b001001011001,
        0b010011100111,
        0b101001010011,
        0b101000001101,
        0b000100001111,
        0b111111011111,
        0b000101001010,
        0b011011010010,
        0b101010000101,
        0b001101001010,
        0b000001100001,
        0b010010110100,
        0b010011000010,
        0b111110011111,
        0b010011111011,
        0b111100010111,
        0b000000010100,
        0b110011011111,
        0b000100110100,
        0b011001000101,
        0b010110100011,
        0b100011010011,
        0b010111001110,
        0b100101010000,
        0b001001010110,
        0b110000001000,
        0b101111110001,
        0b101101001101,
        0b010100000100,
        0b100101111010,
        0b001111011010,
        0b101110011111,
        0b110000010111,
        0b100011011110,
        0b000000100111,
        0b010111010111,
        0b100001011011,
        0b101001011010,
        0b001111001111,
        0b110101100001,
        0b010010011101,
        0b010010101011,
        0b111100111111,
        0b000110110000,
        0b000100100110,
        0b100100111001,
        0b011100101001,
        0b110011100000,
        0b100111111001,
        0b101011110100,
        0b001010101001,
        0b011111010110,
        0b110010110100,
        0b111101011000,
        0b100110011000,
        0b100100001000,
        0b010111100001,
        0b111001110011,
        0b100110100011,
        0b011000011001,
        0b000000000100,
        0b000010110010,
        0b011010111000,
        0b010111010000,
        0b011000100010,
        0b100110011100,
        0b101110011000,
        0b001100100010,
        0b010100111100,
        0b001101101010,
        0b110111100001,
        0b101110010000,
        0b000000110100,
        0b111010000001,
        0b010011001001,
        0b000010001100,
        0b001001101101,
        0b010010000011,
        0b011111100110,
        0b111010000101,
        0b110101110000,
        0b010011001100,
        0b100101111111,
        0b000010001110,
        0b111100001011,
        0b111100110111,
        0b110101000100,
        0b000111110111,
        0b011100011101,
        0b110011101101,
        0b000101100111,
        0b101100101100,
        0b111110101000,
        0b000110101011,
        0b010110011110,
        0b000000111000,
        0b100000000100,
        0b110101101000,
        0b101011100101,
        0b101111000000,
        0b110111000001,
        0b010001100110,
        0b110010010100,
        0b111011010111,
        0b101110011110,
        0b111100110000,
        0b001101110010,
        0b010010100001,
        0b000111100110,
        0b010000110000,
        0b001011010000,
        0b110011010100,
    ];

    let mut gamma_rate = 0;
    for index in 0..12 {
        if filter_with_enabled_bit_at_index(&numbers, index).len() >= numbers.len() / 2 {
            gamma_rate |= 1 << index;
        }
    }
    let epsilon_rate = (!gamma_rate) & 0xfff;
    println!("Part One: {}", gamma_rate * epsilon_rate);

    let mut oxygen_generator_numbers = numbers.clone();
    let mut co2_scrubber_numbers = numbers;
    for index in (0..12).rev() {
        if oxygen_generator_numbers.len() > 1 {
            let enabled_numbers =
                filter_with_enabled_bit_at_index(&oxygen_generator_numbers, index);
            let disabled_numbers =
                filter_with_disabled_bit_at_index(&oxygen_generator_numbers, index);
            oxygen_generator_numbers = if enabled_numbers.len() >= disabled_numbers.len() {
                enabled_numbers
            } else {
                disabled_numbers
            };
        }
        if co2_scrubber_numbers.len() > 1 {
            let enabled_numbers = filter_with_enabled_bit_at_index(&co2_scrubber_numbers, index);
            let disabled_numbers = filter_with_disabled_bit_at_index(&co2_scrubber_numbers, index);
            co2_scrubber_numbers = if enabled_numbers.len() >= disabled_numbers.len() {
                disabled_numbers
            } else {
                enabled_numbers
            };
        }
    }
    assert_eq!(oxygen_generator_numbers.len(), 1);
    assert_eq!(co2_scrubber_numbers.len(), 1);
    println!(
        "Part Two: {}",
        oxygen_generator_numbers[0] * co2_scrubber_numbers[0]
    );
}

fn filter_with_enabled_bit_at_index(numbers: &Vec<u32>, index: u8) -> Vec<u32> {
    numbers
        .iter()
        .filter(|number| *number & (1 << index) != 0)
        .copied()
        .collect()
}

fn filter_with_disabled_bit_at_index(numbers: &Vec<u32>, index: u8) -> Vec<u32> {
    numbers
        .iter()
        .filter(|number| *number & (1 << index) == 0)
        .copied()
        .collect()
}
