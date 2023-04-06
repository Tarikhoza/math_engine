use math_engine::parser::{Parsable, Parser};
#[test]
fn basic_operations() {
    assert_eq!("2", Parser::new("4/2").parse().simplify().to_tex());
    assert_eq!("20", Parser::new("40/2").parse().simplify().to_tex());
    assert_eq!("12", Parser::new("2*3*2").parse().simplify().to_tex());
    assert_eq!("6", Parser::new("2+2+2").parse().simplify().to_tex());
    assert_eq!("4", Parser::new("2+2").parse().simplify().to_tex());
    assert_eq!("3", Parser::new("1+2").parse().simplify().to_tex());
    assert_eq!("91", Parser::new("56+7*5").parse().simplify().to_tex());
    assert_eq!("56", Parser::new("11*8-8*4").parse().simplify().to_tex());
    assert_eq!(
        "2131",
        Parser::new("76*46-35*39").parse().simplify().to_tex()
    );
    assert_eq!("10", Parser::new("54/8+26/8").parse().simplify().to_tex());
    assert_eq!("32", Parser::new("8*7-4*6").parse().simplify().to_tex());
    assert_eq!("38", Parser::new("5*6+2*4").parse().simplify().to_tex());
    assert_eq!("64", Parser::new("12*31-14*22").parse().simplify().to_tex());
    assert_eq!("5168", Parser::new("3821+1347").parse().simplify().to_tex());
    assert_eq!("5758", Parser::new("2512+3246").parse().simplify().to_tex());
    assert_eq!(
        "14005",
        Parser::new("4462+9543").parse().simplify().to_tex()
    );
    assert_eq!(
        "15447",
        Parser::new("9948+5499").parse().simplify().to_tex()
    );
    assert_eq!(
        "13471",
        Parser::new("9745+3726").parse().simplify().to_tex()
    );
    assert_eq!(
        "10778",
        Parser::new("4577+6201").parse().simplify().to_tex()
    );
    assert_eq!("6667", Parser::new("5465+1202").parse().simplify().to_tex());
    assert_eq!(
        "16117",
        Parser::new("8762+7355").parse().simplify().to_tex()
    );
    assert_eq!(
        "11955",
        Parser::new("4578+7377").parse().simplify().to_tex()
    );
    assert_eq!("8857", Parser::new("1057+7800").parse().simplify().to_tex());
    assert_eq!(
        "17478",
        Parser::new("9424+8054").parse().simplify().to_tex()
    );
    assert_eq!(
        "17597",
        Parser::new("7873+9724").parse().simplify().to_tex()
    );
    assert_eq!(
        "14874",
        Parser::new("5962+8912").parse().simplify().to_tex()
    );
    assert_eq!("6989", Parser::new("2353+4636").parse().simplify().to_tex());
    assert_eq!("7893", Parser::new("4156+3737").parse().simplify().to_tex());
    assert_eq!(
        "13161",
        Parser::new("4784+8377").parse().simplify().to_tex()
    );
    assert_eq!(
        "12558",
        Parser::new("3269+9289").parse().simplify().to_tex()
    );
    assert_eq!(
        "10799",
        Parser::new("5031+5768").parse().simplify().to_tex()
    );
    assert_eq!(
        "10975",
        Parser::new("8415+2560").parse().simplify().to_tex()
    );
    assert_eq!("8658", Parser::new("7437+1221").parse().simplify().to_tex());
    assert_eq!(
        "10318",
        Parser::new("5786+4532").parse().simplify().to_tex()
    );
    assert_eq!("8825", Parser::new("4204+4621").parse().simplify().to_tex());
    assert_eq!("9013", Parser::new("4438+4575").parse().simplify().to_tex());
    assert_eq!(
        "10819",
        Parser::new("3432+7387").parse().simplify().to_tex()
    );
    assert_eq!("2474", Parser::new("3821-1347").parse().simplify().to_tex());
    assert_eq!("1266", Parser::new("4512-3246").parse().simplify().to_tex());
    assert_eq!("1919", Parser::new("4462-2543").parse().simplify().to_tex());
    assert_eq!("4449", Parser::new("9948-5499").parse().simplify().to_tex());
    assert_eq!("6019", Parser::new("9745-3726").parse().simplify().to_tex());
    assert_eq!("1688", Parser::new("6201-4513").parse().simplify().to_tex());
    assert_eq!("4263", Parser::new("5465-1202").parse().simplify().to_tex());
    assert_eq!("1407", Parser::new("8762-7355").parse().simplify().to_tex());
    assert_eq!("1201", Parser::new("8578-7377").parse().simplify().to_tex());
    assert_eq!("1257", Parser::new("9057-7800").parse().simplify().to_tex());
    assert_eq!("1370", Parser::new("9424-8054").parse().simplify().to_tex());
    assert_eq!("149", Parser::new("9873-9724").parse().simplify().to_tex());
    assert_eq!("4050", Parser::new("5962-1912").parse().simplify().to_tex());
    assert_eq!("4717", Parser::new("9353-4636").parse().simplify().to_tex());
    assert_eq!("419", Parser::new("4156-3737").parse().simplify().to_tex());
    assert_eq!("1407", Parser::new("4784-3377").parse().simplify().to_tex());
    assert_eq!("3980", Parser::new("7269-3289").parse().simplify().to_tex());
    assert_eq!("163", Parser::new("5931-5768").parse().simplify().to_tex());
    assert_eq!("5855", Parser::new("8415-2560").parse().simplify().to_tex());
    assert_eq!("6216", Parser::new("7437-1221").parse().simplify().to_tex());
    assert_eq!("1254", Parser::new("5786-4532").parse().simplify().to_tex());
    assert_eq!("2583", Parser::new("7204-4621").parse().simplify().to_tex());
    assert_eq!("3263", Parser::new("7838-4575").parse().simplify().to_tex());
    assert_eq!("1045", Parser::new("8432-7387").parse().simplify().to_tex());
    assert_eq!("43730", Parser::new("8746*5").parse().simplify().to_tex());
    assert_eq!("417447", Parser::new("46383*9").parse().simplify().to_tex());
    assert_eq!("314756", Parser::new("78689*4").parse().simplify().to_tex());
    assert_eq!("230120", Parser::new("46024*5").parse().simplify().to_tex());
    assert_eq!("167178", Parser::new("27863*6").parse().simplify().to_tex());
    assert_eq!("538069", Parser::new("76867*7").parse().simplify().to_tex());
    assert_eq!("238569", Parser::new("79523*3").parse().simplify().to_tex());
    assert_eq!("124893", Parser::new("13877*9").parse().simplify().to_tex());
    assert_eq!("170569", Parser::new("24367*7").parse().simplify().to_tex());
    assert_eq!("714078", Parser::new("79342*9").parse().simplify().to_tex());
    assert_eq!("80910", Parser::new("13485*6").parse().simplify().to_tex());
    assert_eq!("437094", Parser::new("48566*9").parse().simplify().to_tex());
    assert_eq!("875079", Parser::new("97231*9").parse().simplify().to_tex());
    assert_eq!("128552", Parser::new("32138*4").parse().simplify().to_tex());
    assert_eq!("166173", Parser::new("23739*7").parse().simplify().to_tex());
    assert_eq!("227874", Parser::new("37979*6").parse().simplify().to_tex());
    assert_eq!("30732", Parser::new("15366*2").parse().simplify().to_tex());
    assert_eq!("229119", Parser::new("76373*3").parse().simplify().to_tex());
    assert_eq!("48228", Parser::new("12057*4").parse().simplify().to_tex());
    assert_eq!("685447", Parser::new("97921*7").parse().simplify().to_tex());
    assert_eq!("97520", Parser::new("48760*2").parse().simplify().to_tex());
    assert_eq!("310344", Parser::new("38793*8").parse().simplify().to_tex());
    assert_eq!("135736", Parser::new("16967*8").parse().simplify().to_tex());
    assert_eq!("74016", Parser::new("24672*3").parse().simplify().to_tex());
    assert_eq!(
        "5146887",
        Parser::new("3821*1347").parse().simplify().to_tex()
    );
    assert_eq!(
        "14645952",
        Parser::new("4512*3246").parse().simplify().to_tex()
    );
    assert_eq!(
        "11346866",
        Parser::new("4462*2543").parse().simplify().to_tex()
    );
    assert_eq!(
        "54704052",
        Parser::new("9948*5499").parse().simplify().to_tex()
    );
    assert_eq!(
        "36309870",
        Parser::new("9745*3726").parse().simplify().to_tex()
    );
    assert_eq!(
        "27985113",
        Parser::new("6201*4513").parse().simplify().to_tex()
    );
    assert_eq!(
        "6568930",
        Parser::new("5465*1202").parse().simplify().to_tex()
    );
    assert_eq!(
        "64444510",
        Parser::new("8762*7355").parse().simplify().to_tex()
    );
    assert_eq!(
        "63279906",
        Parser::new("8578*7377").parse().simplify().to_tex()
    );
    assert_eq!(
        "70644600",
        Parser::new("9057*7800").parse().simplify().to_tex()
    );
    assert_eq!(
        "75900896",
        Parser::new("9424*8054").parse().simplify().to_tex()
    );
    assert_eq!(
        "96005052",
        Parser::new("9873*9724").parse().simplify().to_tex()
    );
    assert_eq!(
        "11399344",
        Parser::new("5962*1912").parse().simplify().to_tex()
    );
    assert_eq!(
        "43360508",
        Parser::new("9353*4636").parse().simplify().to_tex()
    );
    assert_eq!(
        "15530972",
        Parser::new("4156*3737").parse().simplify().to_tex()
    );
    assert_eq!(
        "16155568",
        Parser::new("4784*3377").parse().simplify().to_tex()
    );
    assert_eq!(
        "23907741",
        Parser::new("7269*3289").parse().simplify().to_tex()
    );
    assert_eq!(
        "34210008",
        Parser::new("5931*5768").parse().simplify().to_tex()
    );
    assert_eq!(
        "21542400",
        Parser::new("8415*2560").parse().simplify().to_tex()
    );
    assert_eq!(
        "9080577",
        Parser::new("7437*1221").parse().simplify().to_tex()
    );
    assert_eq!(
        "26222152",
        Parser::new("5786*4532").parse().simplify().to_tex()
    );
    assert_eq!(
        "33289684",
        Parser::new("7204*4621").parse().simplify().to_tex()
    );
    assert_eq!(
        "35858850",
        Parser::new("7838*4575").parse().simplify().to_tex()
    );
    assert_eq!(
        "62287184",
        Parser::new("8432*7387").parse().simplify().to_tex()
    );
    assert_eq!("2365", Parser::new("7095/3").parse().simplify().to_tex());
    assert_eq!("2256", Parser::new("4512/2").parse().simplify().to_tex());
    assert_eq!("821", Parser::new("7389/9").parse().simplify().to_tex());
    assert_eq!("1234", Parser::new("9872/8").parse().simplify().to_tex());
    assert_eq!("2354", Parser::new("9416/4").parse().simplify().to_tex());
    assert_eq!("843", Parser::new("7587/9").parse().simplify().to_tex());
    assert_eq!("2526", Parser::new("7578/3").parse().simplify().to_tex());
    assert_eq!("1234", Parser::new("8638/7").parse().simplify().to_tex());
    assert_eq!("8231", Parser::new("8231/1").parse().simplify().to_tex());
    assert_eq!("753", Parser::new("6024/8").parse().simplify().to_tex());
    assert_eq!("456", Parser::new("6840/15").parse().simplify().to_tex());
    assert_eq!("321", Parser::new("7704/24").parse().simplify().to_tex());
    assert_eq!("428", Parser::new("2568/6").parse().simplify().to_tex());
    assert_eq!("719", Parser::new("5033/7").parse().simplify().to_tex());
    assert_eq!("1895", Parser::new("9475/5").parse().simplify().to_tex());
    assert_eq!("753", Parser::new("6024/8").parse().simplify().to_tex());
    assert_eq!("1039", Parser::new("7273/7").parse().simplify().to_tex());
    assert_eq!("526", Parser::new("3682/7").parse().simplify().to_tex());
    assert_eq!("3369", Parser::new("6738/2").parse().simplify().to_tex());
    assert_eq!("1058", Parser::new("8464/8").parse().simplify().to_tex());
    assert_eq!("951", Parser::new("4755/5").parse().simplify().to_tex());
    assert_eq!("852", Parser::new("5112/6").parse().simplify().to_tex());
    assert_eq!("789", Parser::new("9468/12").parse().simplify().to_tex());
    assert_eq!("369", Parser::new("5904/16").parse().simplify().to_tex());
    assert_eq!(
        "61742",
        Parser::new("3821+1347*43").parse().simplify().to_tex()
    );
    assert_eq!(
        "4180",
        Parser::new("4525-2070/6").parse().simplify().to_tex()
    );
    assert_eq!(
        "5100",
        Parser::new("8124+1347-4371").parse().simplify().to_tex()
    );
    assert_eq!(
        "6446",
        Parser::new("7124-2070+1392").parse().simplify().to_tex()
    );
    assert_eq!(
        "60063",
        Parser::new("4284/2+1347*43").parse().simplify().to_tex()
    );
    assert_eq!(
        "1000",
        Parser::new("8285/5-5256/8").parse().simplify().to_tex()
    );
    assert_eq!(
        "8588",
        Parser::new("3482*3-7432/4").parse().simplify().to_tex()
    );
    assert_eq!(
        "12361",
        Parser::new("8285*8/5-5256+4361")
            .parse()
            .simplify()
            .to_tex()
    );
    assert_eq!(
        "10981",
        Parser::new("1288*2*4+5416/4/2").parse().simplify().to_tex()
    );
    assert_eq!(
        "12139",
        Parser::new("4265*3-3236*2+1454*4")
            .parse()
            .simplify()
            .to_tex()
    );
    //   >>) 822 · 9 · 6 : 3 − 632 · 11 : 2 = 11320 l) 4265 : 5 + 6438 : 6 + 17848 : 8 = 4157
    //   >>ufgabe 12:
    //   >>) 1203 + 6799 + 5684 + 2156 + 9852 = 25694
    //   >>) 9982 − 595 − 651 − 3197 − 1637 = 3902
    //   >>) 11 · 4 · 6 · 7 · 3 = 5544
    //   >>) 1587 + 9613 + 9477 + 2674 + 2987 = 26338
    //   >>) 8745 − 2971 − 841 − 1052 − 681 = 3200
    //   >> ) 5 · 2 · 4 · 9 · 7 = 2520
    //   >>) 8461 + 5625 + 1098 + 6950 + 8509 = 30643
    //   >>) 7894 − 412 − 753 − 951 − 456 = 5322
    //   >>) 12 · 3 · 5 · 10 · 9 = 16200
    //   >>
    //   >>
    //   >>
    //   >>ommatzsch Repetitorium der Mathematik
    //   >>) 2 + 6 · 3 − 5 = 15 b) 12 − 20 : 5 + 8 · 2 = 24
    //   >>) 2 · 8 : 4 + 2 − 6 : 2 = 3 d) 63 − 4 · 7 − 3 · 3 + 15 = 41
    //   >>) 12 · 4 − 6 · 5 + 33 · 7 − 43 · 4 = 77 f ) 10 · 45 − 15 : 5 · 60 + 34 = 304
    //   >>) 7 + 3 · 17 + 56 · 6 − 123 = 271 h) 55 · 48 : 4 : 3 − 12 · 3 = 184
    //   >>) 54 · 11 − 9 · 47 − 72 : 2 · 3 = 63 j) 74 · 3 − 4 · 8 + 32 · 5 = 350
    //   >>) 2 · 4 · 5 · 6 − 12 · 5 = 180 l) 864 : 8 : 3 + 23 · 7 − 12 · 4 = 141
    //   >>) 142 · 13 − 24 · 82 : 4 + 91 · 12 = 2446 n) 1249 · 34 − 2135 : 5 + 235 · 5 = 43214
    //   >>ufgabe 14: Berechne das Ergebnis.
    //   >>) 2 · 4 + 6 · 3 − 2 · 11 = 4 b) 4 · 12 : 3 − 5 · 2 + 3 = 9
    //   >>) 75 : 5 − 5 + 72 : 8 · 3 = 37 d) 10 · 9 − 3 · 2 · 9 = 36
    //   >>) 6 + 6 · 5 · 9 − 3 · 6 = 258 f ) 81 : 3 : 3 + 6 · 7 − 5 · 7 = 16
    //   >>) 55 − 3 · 7 + 32 − 4 · 3 · 2 = 42 h) 15 · 4 − 9 · 3 − 4 · 5 : 2 = 23
    //   >>) 60 : 5 : 6 + 24 · 5 − 7 · 9 = 59 j) 11 · 99 − 2 · 6 : 4 · 8 = 1065
    //   >>) 54 : 9 · 3 + 7 · 5 · 3 = 123 l) 5 · 5 · 4 : 10 + 9 · 4 : 3 = 22
    //   >>) 42 − 11 · 3 + 3 · 12 = 45 n) 49 · 5 : 7 − 6 · 2 + 4 · 7 = 51
    //   >>ufgabe 15: Berechne das Ergebnis.
    //   >>) 4731 + 3772 + 9141 + 1156 + 11278 + 3731 + 7542 + 1127 + 9346 = 51824
    //   >>) 156 + 59342 + 3482 + 45642 + 284921 + 6432 + 25 + 27381 + 22833 = 450214
    //   >>) 826265 − 1661 − 5683 − 34667 − 3727 − 8790 − 5204 − 4267 = 76226
    //   >>) 438693 − 2362 − 2677 − 6431 − 6530 − 3256 − 1354 − 7532 − 2637 = 405914
    //   >>) 36377 + 2367 + 2326 − 2362 − 6432 + 3467 − 9032 + 9463 − 7524 − 7345 = 21305
    //   >> ) 89349 + 3256 − 8347 + 8939 · 11 − 4578 + 3250 − 6784 · 5 = 147339
    //   >>) 3422 · 312 − 26806 + 6 · 7846 − 92357 + 8356 · 8 − 4362 = 1058063
    //   >>) 29857 − 23658 + 26 · 366 − 543 + 23568 − 485 · 38 − 436 + 326 − 2666 − 735 = 16799
    assert_eq!(
        "16799",
        Parser::new("29857-23658+26*366-543+23568-485*38-436+326-2666-735")
            .parse()
            .simplify()
            .to_tex()
    );
}

#[test]
fn decimal() {
    assert_eq!(
        "5.5",
        Parser::new("1.3*7-4.5*0.8").parse().simplify().to_tex()
    );
    assert_eq!(
        "22.3",
        Parser::new("5.2*7-3*4.7").parse().simplify().to_tex()
    );
    assert_eq!(
        "8.64",
        Parser::new("1.2*5.7+9*0.2").parse().simplify().to_tex()
    );
}

#[test]
fn braces() {
    assert_eq!(
        "619",
        Parser::new("45+7*(98-144/9)").parse().simplify().to_tex()
    );
    assert_eq!(
        "655",
        Parser::new("(13*21-112)*4+264/24")
            .parse()
            .simplify()
            .to_tex()
    );
    assert_eq!(
        "13",
        Parser::new("(6*5+9*8)/3-3*7").parse().simplify().to_tex()
    );
}
