use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
enum Packets {
    V4(Packet4),
    VX(PacketX),
}

#[derive(Debug, Eq, PartialEq)]
struct Packet4 {
    version: usize,
    typ: usize,
    val: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct PacketX {
    version: usize,
    typ: usize,
    packets: Vec<Packets>,
}

fn main() {
    let input_path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = input_reader::read(&input_path);
    let binary = to_binary(&input.trim());
    let packets = &read_packet(&binary).0;

    let now = Instant::now();
    dbg!(count_packet(packets));

    println!(
        "Solution took {} ms",
        now.elapsed().as_micros() as f64 / 1000.0
    );
}

fn read_packet(bit_string: &str) -> (Packets, usize) {
    let version = get_version(&bit_string);
    let typ = get_type(&bit_string);
    match typ {
        4 => {
            let (val, index) = read_bits(bit_string);
            (Packets::V4(Packet4 { version, typ, val }), index)
        }
        _ => {
            let l = get_length_typ(&bit_string);

            match l {
                0 => {
                    let relevant_bits = get_length_sub_15(bit_string);

                    let (packets, i) = read_sub_packets_bit_size(&bit_string[22..], relevant_bits);

                    return (
                        Packets::VX(PacketX {
                            version,
                            typ,
                            packets,
                        }),
                        i + 22,
                    );
                }
                1 => {
                    let relevant_bits = get_length_sub_11(bit_string);

                    let (packets, i) = read_sub_packets_amount(&bit_string[18..], relevant_bits);

                    return (
                        Packets::VX(PacketX {
                            version,
                            typ,
                            packets,
                        }),
                        i + 18,
                    );
                }
                _ => panic!("invalid length typ"),
            }
        }
    }
}

fn to_binary(hex: &str) -> String {
    let digits: Vec<usize> = hex
        .chars()
        .map(|c| c.to_digit(16).unwrap() as usize)
        .collect();

    let bits: Vec<String> = digits.iter().map(|d| format!("{:04b}", d)).collect();
    bits.join("")
}

fn get_version(bit_string: &str) -> usize {
    bit_to_num(&bit_string[0..=2])
}

fn get_type(bit_string: &str) -> usize {
    bit_to_num(&bit_string[3..=5])
}

fn get_length_typ(bit_string: &str) -> usize {
    bit_to_num(&bit_string[6..=6])
}

fn get_length_sub_15(bit_string: &str) -> usize {
    bit_to_num(&bit_string[7..22])
}
fn get_length_sub_11(bit_string: &str) -> usize {
    bit_to_num(&bit_string[7..18])
}

fn bit_to_num(bit_string: &str) -> usize {
    let str = format!("0{}", &bit_string);
    usize::from_str_radix(&str, 2).unwrap()
}

fn read_bits(bit_string: &str) -> (usize, usize) {
    let range = 0..bit_string.len() - 6;

    let mut string = String::new();
    for i in range.step_by(5) {
        let index = i + 6;
        let bit_size = 5;

        let s = &bit_string[index..index + bit_size];
        string += &s[1..].to_string();

        if &s[0..1] == "0" {
            return (bit_to_num(&string), index + bit_size);
        }
    }

    panic!("no number")
}

fn read_sub_packets_bit_size(bit_string: &str, size: usize) -> (Vec<Packets>, usize) {
    let mut index = 0;
    let mut vec = vec![];
    while index < size {
        let (s, i) = read_packet(&bit_string[index..]);
        vec.push(s);
        index += i;
    }

    (vec, index)
}

fn read_sub_packets_amount(bit_string: &str, amount: usize) -> (Vec<Packets>, usize) {
    let mut index = 0;
    let mut vec = vec![];

    for a in 0..amount {
        let (s, i) = read_packet(&bit_string[index..]);
        vec.push(s);
        index += i;
    }

    (vec, index)
}

fn count_version(packets: &Packets) -> usize {
    match packets {
        Packets::V4(packet) => packet.version,
        Packets::VX(packet) => {
            let sum = packet.packets.iter().map(count_version).sum::<usize>() as usize;
            sum + packet.version
        }
    }
}

fn count_packet(packets: &Packets) -> usize {
    match packets {
        Packets::V4(packet) => packet.val,
        Packets::VX(packet) => match packet.typ {
            0 => packet.packets.iter().map(count_packet).sum::<usize>() as usize,
            1 => packet.packets.iter().map(count_packet).product::<usize>() as usize,
            2 => packet.packets.iter().map(count_packet).min().unwrap(),
            3 => packet.packets.iter().map(count_packet).max().unwrap(),
            5 => {
                let first = &packet.packets[0];
                let second = &packet.packets[1];
                if count_packet(first) > count_packet(second) {
                    return 1;
                } else {
                    return 0;
                }
            }
            6 => {
                let first = &packet.packets[0];
                let second = &packet.packets[1];
                if count_packet(first) < count_packet(second) {
                    return 1;
                } else {
                    return 0;
                }
            }
            7 => {
                let first = &packet.packets[0];
                let second = &packet.packets[1];
                if count_packet(first) == count_packet(second) {
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => panic!("invariant version"),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        count_packet, count_version, get_length_sub_11, get_length_sub_15, get_length_typ,
        get_type, get_version, read_bits, read_packet, read_sub_packets_amount,
        read_sub_packets_bit_size, to_binary, Packet4, PacketX, Packets,
    };

    #[test]
    fn it_should_parse_hex_to_binary() {
        // Arrange
        let row = "D2FE28";

        // Act
        let result = to_binary(row);

        // Assert
        assert_eq!(result, String::from("110100101111111000101000"));
    }

    #[test]
    fn it_should_parse_packet_4() {
        // Arrange
        let row = "110100101111111000101000";

        // Act
        let result = read_packet(row);

        // Assert
        assert_eq!(
            result,
            (
                Packets::V4(Packet4 {
                    version: 6,
                    typ: 4,
                    val: 2021
                }),
                1
            )
        );
    }

    #[test]
    fn it_should_get_type_from_bit_string() {
        // Arrange
        let row = "110100101111111000101000";

        // Act
        let result = get_version(row);

        // Assert
        assert_eq!(result, 6);
    }

    #[test]
    fn it_should_get_id_from_bit_string() {
        // Arrange
        let row = "110100101111111000101000";

        // Act
        let result = get_type(row);

        // Assert
        assert_eq!(result, 4);
    }

    #[test]
    fn it_should_read_bits() {
        // Arrange
        let row = "110100101111111000101000";

        // Act
        let result = read_bits(row);

        // Assert
        assert_eq!(result, (2021, 21));
    }
    #[test]
    fn it_should_read_length_type() {
        // Arrange
        let row = "00111000000000000110111101000101001010010001001000000000";

        // Act
        let result = get_length_typ(row);

        // Assert
        assert_eq!(result, 0);
    }

    #[test]
    fn it_should_read_length_sub_15() {
        // Arrange
        let row = "00111000000000000110111101000101001010010001001000000000";

        // Act
        let result = get_length_sub_15(row);

        // Assert
        assert_eq!(result, 27);
    }

    #[test]
    fn it_should_read_length_sub_11() {
        // Arrange
        let row = "1101010000001100100000100011000001100000";

        // Act
        let result = get_length_sub_11(row);

        // Assert
        assert_eq!(result, 3);
    }

    #[test]
    fn it_should_read_sub_packets() {
        // Arrange
        let row = "1101000101001010010001001000000000";

        // Act
        let result = read_sub_packets_bit_size(row, 27);

        // Assert
        assert_eq!(
            result,
            (
                vec![
                    Packets::V4(Packet4 {
                        version: 6,
                        typ: 4,
                        val: 10
                    }),
                    Packets::V4(Packet4 {
                        version: 2,
                        typ: 4,
                        val: 20
                    })
                ],
                27
            )
        );
    }

    #[test]
    fn it_should_read_sub_packets2() {
        // Arrange
        let row = "01010000001100100000100011000001100000";

        // Act
        let result = read_sub_packets_amount(row, 3);

        // Assert
        assert_eq!(
            result,
            (
                vec![
                    Packets::V4(Packet4 {
                        version: 2,
                        typ: 4,
                        val: 1
                    }),
                    Packets::V4(Packet4 {
                        version: 4,
                        typ: 4,
                        val: 2
                    }),
                    Packets::V4(Packet4 {
                        version: 1,
                        typ: 4,
                        val: 3
                    })
                ],
                33
            )
        );
    }

    #[test]
    fn it_should_read_sub_packets3() {
        // Arrange
        let row = "11101110000000001101010000001100100000100011000001100000";

        // Act
        let result = read_packet(row);

        // Assert
        assert_eq!(
            result,
            (
                Packets::VX(PacketX {
                    packets: vec![
                        Packets::V4(Packet4 {
                            version: 2,
                            typ: 4,
                            val: 1
                        }),
                        Packets::V4(Packet4 {
                            version: 4,
                            typ: 4,
                            val: 2
                        }),
                        Packets::V4(Packet4 {
                            version: 1,
                            typ: 4,
                            val: 3
                        })
                    ],
                    version: 7,
                    typ: 3
                },),
                33
            )
        );
    }

    #[test]
    fn it_should_work_with_examples() {
        // Arrange
        let row:Vec<(&str, usize)> = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (str, expected_result) in row {
            // Act
            let result = count_packet(&read_packet(&to_binary(str)).0);

            // Assert
            assert_eq!(result, expected_result)
        }
    }
}
