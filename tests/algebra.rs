use math_engine::lexer::Tokenisable;
use math_engine::math::simplifiable::Simplifiable;
use math_engine::parser::{Parsable, Parser};

macro_rules! parser_eq {
    ($input:expr, $expected:expr) => {
        assert_eq!(
            Parser::new($input.tokenise().unwrap())
                .parse()
                .unwrap()
                .0
                .simplify()
                .simplify()
                .simplify()
                .simplify()
                .to_tex(),
            $expected
        );
    };
}

#[test]
fn basic_operations() {
    parser_eq!("4/2", "2");
    parser_eq!("40/2", "20");
    parser_eq!("2*3*2", "12");
    parser_eq!("2+2+2", "6");
    parser_eq!("2+2", "4");
    parser_eq!("1+2", "3");
    parser_eq!("56+7*5", "91");
    parser_eq!("11*8-8*4", "56");
    parser_eq!("76*46-35*39", "2131");
    parser_eq!("54/8+26/8", "10");
    parser_eq!("8*7-4*6", "32");
    parser_eq!("5*6+2*4", "38");
    parser_eq!("12*31-14*22", "64");
    parser_eq!("3821+1347", "5168");
    parser_eq!("2512+3246", "5758");
    parser_eq!("4462+9543", "14005");
    parser_eq!("9948+5499", "15447");
    parser_eq!("9745+3726", "13471");
    parser_eq!("4577+6201", "10778");
    parser_eq!("5465+1202", "6667");
    parser_eq!("8762+7355", "16117");
    parser_eq!("4578+7377", "11955");
    parser_eq!("1057+7800", "8857");
    parser_eq!("5962+8912", "14874");
    parser_eq!("2353+4636", "6989");
    parser_eq!("4156+3737", "7893");
    parser_eq!("4784+8377", "13161");
    parser_eq!("3269+9289", "12558");
    parser_eq!("5031+5768", "10799");
    parser_eq!("8415+2560", "10975");
    parser_eq!("7437+1221", "8658");
    parser_eq!("5786+4532", "10318");
    parser_eq!("4204+4621", "8825");
    parser_eq!("4438+4575", "9013");
    parser_eq!("3432+7387", "10819");
    parser_eq!("3821-1347", "2474");
    parser_eq!("4512-3246", "1266");
    parser_eq!("4462-2543", "1919");
    parser_eq!("9948-5499", "4449");
    parser_eq!("9745-3726", "6019");
    parser_eq!("6201-4513", "1688");
    parser_eq!("5465-1202", "4263");
    parser_eq!("8762-7355", "1407");
    parser_eq!("8578-7377", "1201");
    parser_eq!("9057-7800", "1257");
    parser_eq!("9424-8054", "1370");
    parser_eq!("9873-9724", "149");
    parser_eq!("5962-1912", "4050");
    parser_eq!("9353-4636", "4717");
    parser_eq!("4156-3737", "419");
    parser_eq!("4784-3377", "1407");
    parser_eq!("7269-3289", "3980");
    parser_eq!("5931-5768", "163");
    parser_eq!("7437-1221", "6216");
    parser_eq!("5786-4532", "1254");
    parser_eq!("7204-4621", "2583");
    parser_eq!("7838-4575", "3263");
    parser_eq!("8746*5", "43730");
    parser_eq!("46383*9", "417447");
    parser_eq!("78689*4", "314756");
    parser_eq!("46024*5", "230120");
    parser_eq!("27863*6", "167178");
    parser_eq!("76867*7", "538069");
    parser_eq!("79523*3", "238569");
    parser_eq!("13877*9", "124893");
    parser_eq!("24367*7", "170569");
    parser_eq!("79342*9", "714078");
    parser_eq!("13485*6", "80910");
    parser_eq!("48566*9", "437094");
    parser_eq!("97231*9", "875079");
    parser_eq!("32138*4", "128552");
    parser_eq!("23739*7", "166173");
    parser_eq!("37979*6", "227874");
    parser_eq!("15366*2", "30732");
    parser_eq!("76373*3", "229119");
    parser_eq!("12057*4", "48228");
    parser_eq!("97921*7", "685447");
    parser_eq!("48760*2", "97520");
    parser_eq!("38793*8", "310344");
    parser_eq!("16967*8", "135736");
    parser_eq!("24672*3", "74016");
    parser_eq!("3821*1347", "5146887");
    parser_eq!("4512*3246", "14645952");
    parser_eq!("4462*2543", "11346866");
    parser_eq!("9948*5499", "54704052");
    parser_eq!("9745*3726", "36309870");
    parser_eq!("6201*4513", "27985113");
    parser_eq!("5465*1202", "6568930");
    parser_eq!("8762*7355", "64444510");
    parser_eq!("8578*7377", "63279906");
    parser_eq!("9057*7800", "70644600");
    parser_eq!("9424*8054", "75900896");
    parser_eq!("9873*9724", "96005052");
    parser_eq!("5962*1912", "11399344");
    parser_eq!("9353*4636", "43360508");
    parser_eq!("4156*3737", "15530972");
    parser_eq!("4784*3377", "16155568");
    parser_eq!("7269*3289", "23907741");
    parser_eq!("5931*5768", "34210008");
    parser_eq!("8415*2560", "21542400");
    parser_eq!("8415*2560", "21542400");
    parser_eq!("7437*1221", "9080577");
    parser_eq!("5786*4532", "26222152");
    parser_eq!("7204*4621", "33289684");
    parser_eq!("7838*4575", "35858850");
    parser_eq!("8432*7387", "62287184");
    parser_eq!("8432*7387", "62287184");
    parser_eq!("7095/3", "2365");
    parser_eq!("4512/2", "2256");
    parser_eq!("7389/9", "821");
    parser_eq!("9872/8", "1234");
    parser_eq!("9416/4", "2354");
    parser_eq!("7587/9", "843");
    parser_eq!("7578/3", "2526");
    parser_eq!("8638/7", "1234");
    parser_eq!("8231/1", "8231");
    parser_eq!("6024/8", "753");
    parser_eq!("6840/15", "456");
    parser_eq!("7704/24", "321");
    parser_eq!("2568/6", "428");
    parser_eq!("5033/7", "719");
    parser_eq!("9475/5", "1895");
    parser_eq!("6024/8", "753");
    parser_eq!("7273/7", "1039");
    parser_eq!("3682/7", "526");
    parser_eq!("6738/2", "3369");
    parser_eq!("8464/8", "1058");
    parser_eq!("4755/5", "951");
    parser_eq!("5112/6", "852");
    parser_eq!("9468/12", "789");
    parser_eq!("5904/16", "369");
    parser_eq!("3821+1347*43", "61742");
    parser_eq!("4525-2070/6", "4180");
    parser_eq!("8124+1347-4371", "5100");
    parser_eq!("7124-2070+1392", "6446");
    parser_eq!("4284/2+1347*43", "60063");
    parser_eq!("8285/5-5256/8", "1000");
    parser_eq!("3482*3-7432/4", "8588");
    parser_eq!("8285*8/5-5256+4361", "12361");
    parser_eq!("1288*2*4+5416/4/2", "10981");
    parser_eq!("4265*3-3236*2+1454*4", "12139");
    parser_eq!("822*9*6*3-632*11/2", "129688");
    parser_eq!("4265/5+6438/6+17848/8", "4157");
    parser_eq!("1203+6799+5684+2156+9852", "25694");
    parser_eq!("9982-595-651-3197-1637", "3902");
    parser_eq!("11*4*6*7*3", "5544");
    parser_eq!("1587+9613+9477+2674+2987", "26338");
    parser_eq!("8745-2971-841-1052-681", "3200");
    parser_eq!("5*2*4*9*7", "2520");
    parser_eq!("8461+5625+1098+6950+8509", "30643");
    parser_eq!("7894-412-753-951-456", "5322");
    parser_eq!("12*3*5*10*9", "16200");
    parser_eq!("2+6*3-5", "15");
    parser_eq!("12-20/5+8*2", "24");
    parser_eq!("2*8/4+2-6/2", "3");
    parser_eq!("6304*7-3*3+15", "44134");
    parser_eq!("12*4-6*5+33*7-43*4", "77");
    parser_eq!("10*45-15/5*60+34", "304");
    parser_eq!("7+3*17+56*6-123", "271");
    parser_eq!("55*48/4/3-12*3", "184");
    parser_eq!("54*11-9*47-72/2*3", "63");
    parser_eq!("74*3-4*8+32*5", "350");
    parser_eq!("2*4*5*6-12*5", "180");
    parser_eq!("864/8/3+23*7-12*4", "149");
    parser_eq!("142*13-24*82/4+91*12", "2446");
    parser_eq!("2*4+6*3-2*11", "4");
    parser_eq!("4*12/3-5*2+3", "9");
    parser_eq!("75/5-5+72/8*3", "37");
    parser_eq!("10*9-3*2*9", "36");
    parser_eq!("6+6*5*9-3*6", "258");
    parser_eq!("81/3/3+6*7-5*7", "16");
    parser_eq!("55-3*7+32-4*3*2", "42");
    parser_eq!("15*4-9*3-4*5/2", "23");
    parser_eq!("60/5/6+24*5-7*9", "59");
    parser_eq!("11*99-2*6/4*8", "1065");
    parser_eq!("54/9*3+7*5*3", "123");
    parser_eq!("5*5*4/10+9*4/3", "22");
    parser_eq!("42-11*3+3*12", "45");
    parser_eq!("49*5/7-6*2+4*7", "51");
    parser_eq!("49*5/7-6*2+4*7", "51");
    parser_eq!("4731+3772+9141+1156+11278+3731+7542+1127+9346", "51824");
    parser_eq!("156+59342+3482+45642+284921+6432+25+27381+22833", "450214");
    parser_eq!("826265-1661-5683-34667-3727-8790-5204-4267", "762266");
    parser_eq!("438693-2362-2677-6431-6530-3256-1354-7532-2637", "405914");
    parser_eq!(
        "36377+2367+2326-2362-6432+3467-9032+9463-7524-7345",
        "21305"
    );
    parser_eq!("89349+3256-8347+8939*11-4578+3250-6784*5", "147339");
    parser_eq!("3422*312-26806+6*7846-92357+8356*8-4362", "1058063");
    parser_eq!(
        "29857-23658+26*366-543+23568-485*38-436+326-2666-735",
        "16799"
    );
    parser_eq!(
        "29857-23658+26*366-543+23568-485*38-436+326-2666-735",
        "16799"
    );
}

#[test]
fn double_operator() {
    parser_eq!("3x+-2x", "x");
    parser_eq!("3x--1x", "4x");
}

#[test]
fn multi_operator() {
    parser_eq!("3x----1x", "4x");
    parser_eq!("3x---1x", "2x");
}

#[test]
fn basic_operations_with_suffix() {
    parser_eq!("a-a", "0");
    parser_eq!("3x+2x", "5x");
    parser_eq!("3x*2x", "6x^{2}");
    parser_eq!("5x+3y+4-2x+8y-7", "3x+11y-3");
    parser_eq!("3x+5a-3a+2x", "2a+5x");
    parser_eq!("69g-5p+16p-7g+11g", "73g+11p");
    parser_eq!("3a+3b+19a-2b+7b", "22a+8b");
    parser_eq!("13a+53b-32b-11b+8a", "21a+10b");
    parser_eq!("5b-7h+3d+18h-4b+12d", "b+15d+11h");
    parser_eq!("12x+8y+23z-5y+13z-9x", "3x+3y+36z");
    parser_eq!("4y+8x+3x-7x+15y", "4x+19y");
    parser_eq!("6k-7u+11k+14u-2k", "15k+7u");
    parser_eq!("6a+5d-3a", "3a+5d");
    parser_eq!("14y+8x-10y-3x", "5x+4y");
    parser_eq!("7g-14h-4g-5h", "3g-19h");
    parser_eq!("82a+66b-49a-38b", "33a+28b");
    parser_eq!("3z-7g+19g+6z", "12g+9z");
    parser_eq!("8a-5f+a-4f", "9a-9f");
    parser_eq!("99k-43h+37h-25k", "74k-6h");
    parser_eq!("82u+13l-44l-77u", "5u-31l");
    parser_eq!("-4z+3r-8r-3z", "-5r-7z");
    parser_eq!("33y-28x+19y+43x-27y", "15x+25y");
    parser_eq!("16c-4c+72d+17c-13d", "29c+59d");
    parser_eq!("25p+8o-9o-22p+5o", "4o+3p");
    parser_eq!("34f+5g-7h+11g+17h-15f", "19f+16g+10h");
    parser_eq!("5a+7b-3a-2b+6a+11a+7b-2a", "17a+12b");
    parser_eq!("9c-4c-6a-12a+36a+3c+a-2c+15c", "19a+21c");
    parser_eq!("a+b+a+b+c+b+a+b-c-a+b+c-a+b-a", "6b+c");
}

#[test]
fn basic_operations_with_suffix_and_braces() {
    parser_eq!("8x+4+3*(2x-3)", "14x-5");
}

#[test]
fn sorting() {
    parser_eq!("2x^{2}+3x+1", "2x^{2}+3x+1");
    parser_eq!("3x+1+2x^{2}", "2x^{2}+3x+1");
}

#[test]
fn decimal() {
    parser_eq!("1.3*7-4.5*0.8", "5.5");
    parser_eq!("5.2*7-3*4.7", "22.3");
    parser_eq!("1.2*5.7+9*0.2", "8.64");
}

#[test]
fn braces() {
    parser_eq!("45+7*(98-144/9)", "619");
    parser_eq!("(13*21-112)*4+264/24", "655");
    parser_eq!("(6*5+9*8)/3-3*7", "13");
    parser_eq!("5*(2364-7*(15*17-97))+178", "6468");
    parser_eq!("(84*34-68*29)*(45*97-643)", "3290248");
    parser_eq!("(11*3+12*7)*(235-8*28)*(27*45-36*25)", "405405");
}
#[test]
fn exponents() {
    parser_eq!("3x^{2}", "3x^{2}");
    parser_eq!("3^{-2}", "\\frac{1}{9}");
    parser_eq!("3^{2}", "9");
    parser_eq!("(3+3)^{2}", "36");
}

//   #[test]
//   fn fractions() {
//       parser_eq!("\\frac{3}{4}+\\frac{2}{5}*\\frac{3}{2}", "\\frac{27}{20}");
//       parser_eq!(
//           "\\frac{6}{7}/\\frac{4}{3}+\\frac{3}{4}/\\frac{6}{5}",
//           "\\frac{71}{56}"
//       );
//       parser_eq!(
//           "\\frac{3}{8}*(\\frac{11}{9}-\\frac{5}{6}*\\frac{1}{3})",
//           "\\frac{17}{48}"
//       );
//       parser_eq!("(\\frac{3}{4}+\\frac{2}{5})*\\frac{3}{2}", "\\frac{69}{40}");
//       parser_eq!(
//           "\\frac{6}{7}/((\\frac{4}{3}+\\frac{3}{4})/\\frac{6}{5})",
//           "\\frac{432}{875}"
//       );
//       parser_eq!(
//           "(\\frac{3}{8}*(\\frac{11}{9}-\\frac{5}{6})+\\frac{5}{8})*\\frac{1}{3}",
//           "\\frac{37}{144}"
//       );
//       parser_eq!("\\frac{5}{6}*\\frac{3}{2}+\\frac{5}{8}*2", "\\frac{5}{2}");
//       parser_eq!(
//           "\\frac{5}{9}/\\frac{2}{3}-\\frac{2}{5}/\\frac{8}{3}",
//           "\\frac{41}{60}"
//       );
//   }
//#[test]
fn fractions_whole() {
    //mixed to basic
    parser_eq!("1\\frac{1}{2}", "\\frac{3}{2}");
    parser_eq!("3\\frac{1}{4}", "\\frac{13}{4}");
    parser_eq!("2\\frac{2}{5}", "\\frac{12}{5}");
    parser_eq!("6\\frac{7}{8}", "\\frac{55}{8}");
    parser_eq!("3\\frac{4}{7}", "\\frac{25}{7}");
    parser_eq!("8\\frac{2}{9}", "\\frac{74}{9}");
    parser_eq!("4\\frac{6}{7}", "\\frac{30}{7}");
    parser_eq!("2\\frac{3}{4}", "\\frac{11}{4}");
    parser_eq!("7\\frac{7}{8}", "\\frac{63}{8}");
    parser_eq!("3\\frac{11}{12}", "\\frac{47}{12}");
    parser_eq!("4\\frac{9}{14}", "\\frac{65}{14}");
    parser_eq!("6\\frac{7}{11}", "\\frac{73}{11}");
    parser_eq!("12\\frac{3}{8}", "\\frac{96}{8}");
    parser_eq!("45\\frac{2}{15}", "\\frac{677}{15}");
    parser_eq!("73\\frac{41}{83}", "\\frac{6063}{83}");
    parser_eq!("0\\frac{573}{991}", "\\frac{573}{991}");
    //basic to mixed
    parser_eq!("\\frac{5}{4}", "1\\frac{1}{4}");
    parser_eq!("\\frac{7}{6}", "1\\frac{1}{6}");
    parser_eq!("\\frac{9}{5}", "1\\frac{4}{5}");
    parser_eq!("\\frac{12}{10}", "1\\frac{2}{10}");
    parser_eq!("\\frac{35}{6}", "5\\frac{5}{6}");
    parser_eq!("\\frac{44}{9}", "4\\frac{8}{9}");
    parser_eq!("\\frac{52}{5}", "10\\frac{2}{5}");
    parser_eq!("\\frac{11}{3}", "3\\frac{2}{3}");
    parser_eq!("\\frac{73}{8}", "9\\frac{1}{8}");
    parser_eq!("\\frac{65}{4}", "16\\frac{1}{4}");
    parser_eq!("\\frac{82}{11}", "7\\frac{5}{11}");
    parser_eq!("\\frac{74}{13}", "5\\frac{9}{13}");
    parser_eq!("\\frac{93}{4}", "23\\frac{1}{4}");
    parser_eq!("\\frac{425}{15}", "28\\frac{5}{15}");
    parser_eq!("\\frac{291}{16}", "18\\frac{3}{16}");
    parser_eq!("\\frac{2455}{48}", "51\\frac{7}{2448}");
}

//#[test]
fn fraction_as_decimal() {
    parser_eq!("\\frac{1}{2}", "0.5");
    parser_eq!("\\frac{1}{3}", "0.33");
    parser_eq!("\\frac{1}{5}", "0.2");
    parser_eq!("\\frac{1}{8}", "0.125");
    parser_eq!("\\frac{1}{4}", "0.25");
    parser_eq!("\\frac{1}{16}", "0.0625");
    parser_eq!("\\frac{3}{4}", "0.75");
    parser_eq!("\\frac{4}{5}", "0.8");
    parser_eq!("\\frac{4}{5}", "0.8");
    parser_eq!("\\frac{2}{3}", "0.66");
    parser_eq!("\\frac{6}{7}", "0.8571");
    parser_eq!("\\frac{1}{12}", "0.833");
    parser_eq!("\\frac{19}{5}", "3.8");
    parser_eq!("\\frac{5}{9}", "0.555");
    parser_eq!("\\frac{11}{6}", "1.833");
    parser_eq!("\\frac{5}{3}", "1.666");
    parser_eq!("\\frac{43}{5}", "8.6");
    parser_eq!("\\frac{55}{2}", "27.5");
    parser_eq!("\\frac{17}{8}", "2.125");
    parser_eq!("\\frac{67}{7}", "9.5714");
    parser_eq!("\\frac{81}{3}", "27");
    parser_eq!("\\frac{55}{7}", "7.857");
    parser_eq!("\\frac{1}{10}", "0.1");
    parser_eq!("\\frac{1}{10}", "0.1");
    parser_eq!("\\frac{1}{100}", "0.01");
    parser_eq!("\\frac{1}{1000}", "0.001");
}

//#[test]
fn log() {
    parser_eq!("\\log_{2}{16}", "4");
    parser_eq!("\\lg{1000}", "3");
    parser_eq!("\\log_{5}{125}", "2");
    parser_eq!("\\lb_{5}{512}", "9");
    parser_eq!("\\ln{e^{9}}", "9");
    parser_eq!("\\log_{5}{125}", "3");
    parser_eq!("\\log_{25}{125}", "\\frac{3}{2}");
    parser_eq!("\\log_{17}{1}", "0");
    parser_eq!("\\log_{3}{81}", "4");
    parser_eq!("\\log_{4}{64}", "3");
    parser_eq!("\\log_{15}{225}", "2");
}

#[test]
fn simple_exponents() {
    parser_eq!("5^{2}*3^{3}-4^{4}", "419");
    parser_eq!("7^{2}+3^{4}+5^{3}", "255");
    parser_eq!("4^{3}*8^{2}", "4096");
    parser_eq!("7^{3}+6^{4}-4^{5}", "615");
    parser_eq!("11^{3}-12^{2}-17^{2}", "898");
    parser_eq!("25^{3}+13^{2}*6^{4}", "234649");
}
