use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

fn main() {
    let scanners = [
        Scanner {
            beacons: vec![
                Beacon {
                    x: 518,
                    y: 548,
                    z: 922,
                },
                Beacon {
                    x: -463,
                    y: 418,
                    z: -234,
                },
                Beacon {
                    x: -47,
                    y: 92,
                    z: 75,
                },
                Beacon {
                    x: 484,
                    y: 596,
                    z: 950,
                },
                Beacon {
                    x: -668,
                    y: -439,
                    z: -315,
                },
                Beacon {
                    x: -311,
                    y: 509,
                    z: -258,
                },
                Beacon {
                    x: 695,
                    y: 613,
                    z: 911,
                },
                Beacon {
                    x: -693,
                    y: 696,
                    z: 407,
                },
                Beacon {
                    x: 824,
                    y: -523,
                    z: -400,
                },
                Beacon {
                    x: 526,
                    y: 863,
                    z: -346,
                },
                Beacon {
                    x: 835,
                    y: -582,
                    z: -299,
                },
                Beacon {
                    x: 732,
                    y: -620,
                    z: -401,
                },
                Beacon {
                    x: -776,
                    y: -340,
                    z: 558,
                },
                Beacon {
                    x: -629,
                    y: 763,
                    z: 528,
                },
                Beacon {
                    x: -769,
                    y: -490,
                    z: -378,
                },
                Beacon {
                    x: -729,
                    y: -489,
                    z: 647,
                },
                Beacon {
                    x: 625,
                    y: 941,
                    z: -377,
                },
                Beacon {
                    x: 740,
                    y: -394,
                    z: 655,
                },
                Beacon {
                    x: -826,
                    y: -336,
                    z: 619,
                },
                Beacon {
                    x: -390,
                    y: 509,
                    z: -275,
                },
                Beacon {
                    x: 578,
                    y: -285,
                    z: 669,
                },
                Beacon {
                    x: -559,
                    y: -474,
                    z: -447,
                },
                Beacon {
                    x: -584,
                    y: 737,
                    z: 551,
                },
                Beacon {
                    x: 509,
                    y: -368,
                    z: 635,
                },
                Beacon {
                    x: 649,
                    y: 903,
                    z: -479,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 672,
                    y: -406,
                    z: 817,
                },
                Beacon {
                    x: 477,
                    y: 386,
                    z: 765,
                },
                Beacon {
                    x: -493,
                    y: -498,
                    z: 726,
                },
                Beacon {
                    x: 643,
                    y: -513,
                    z: 756,
                },
                Beacon {
                    x: -413,
                    y: -474,
                    z: 838,
                },
                Beacon {
                    x: 407,
                    y: 352,
                    z: 716,
                },
                Beacon { x: 97, y: 6, z: 45 },
                Beacon {
                    x: -519,
                    y: 364,
                    z: -714,
                },
                Beacon {
                    x: -625,
                    y: 492,
                    z: 433,
                },
                Beacon {
                    x: 537,
                    y: -664,
                    z: -373,
                },
                Beacon {
                    x: 604,
                    y: 622,
                    z: -584,
                },
                Beacon {
                    x: -404,
                    y: -432,
                    z: 718,
                },
                Beacon {
                    x: -492,
                    y: 260,
                    z: -793,
                },
                Beacon {
                    x: 683,
                    y: 670,
                    z: -638,
                },
                Beacon {
                    x: -567,
                    y: -628,
                    z: -604,
                },
                Beacon {
                    x: 577,
                    y: -696,
                    z: -393,
                },
                Beacon {
                    x: -564,
                    y: 469,
                    z: 564,
                },
                Beacon {
                    x: 132,
                    y: -157,
                    z: -89,
                },
                Beacon {
                    x: -598,
                    y: 471,
                    z: 677,
                },
                Beacon {
                    x: -659,
                    y: -673,
                    z: -606,
                },
                Beacon {
                    x: 441,
                    y: 489,
                    z: 841,
                },
                Beacon {
                    x: 587,
                    y: -444,
                    z: 672,
                },
                Beacon {
                    x: 451,
                    y: -693,
                    z: -375,
                },
                Beacon {
                    x: 685,
                    y: 711,
                    z: -617,
                },
                Beacon {
                    x: -634,
                    y: 259,
                    z: -743,
                },
                Beacon {
                    x: -751,
                    y: -611,
                    z: -648,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -603,
                    y: -567,
                    z: -383,
                },
                Beacon {
                    x: 529,
                    y: -420,
                    z: 881,
                },
                Beacon {
                    x: -785,
                    y: -565,
                    z: -381,
                },
                Beacon {
                    x: -28,
                    y: -3,
                    z: -5,
                },
                Beacon {
                    x: -818,
                    y: 568,
                    z: -551,
                },
                Beacon {
                    x: -824,
                    y: 363,
                    z: -502,
                },
                Beacon {
                    x: 508,
                    y: 716,
                    z: -790,
                },
                Beacon {
                    x: -477,
                    y: -747,
                    z: 647,
                },
                Beacon {
                    x: -422,
                    y: 746,
                    z: 583,
                },
                Beacon {
                    x: -469,
                    y: 572,
                    z: 507,
                },
                Beacon {
                    x: -462,
                    y: -725,
                    z: 465,
                },
                Beacon {
                    x: -136,
                    y: -113,
                    z: 120,
                },
                Beacon {
                    x: -684,
                    y: -602,
                    z: -309,
                },
                Beacon {
                    x: 429,
                    y: 588,
                    z: -742,
                },
                Beacon {
                    x: 589,
                    y: -880,
                    z: -338,
                },
                Beacon {
                    x: 572,
                    y: 603,
                    z: 911,
                },
                Beacon {
                    x: 632,
                    y: 600,
                    z: 732,
                },
                Beacon {
                    x: -479,
                    y: 781,
                    z: 530,
                },
                Beacon {
                    x: 586,
                    y: 561,
                    z: 868,
                },
                Beacon {
                    x: 645,
                    y: -383,
                    z: 881,
                },
                Beacon {
                    x: 590,
                    y: -540,
                    z: 862,
                },
                Beacon {
                    x: 602,
                    y: -866,
                    z: -282,
                },
                Beacon {
                    x: 734,
                    y: -859,
                    z: -287,
                },
                Beacon {
                    x: -487,
                    y: -785,
                    z: 411,
                },
                Beacon {
                    x: 399,
                    y: 664,
                    z: -687,
                },
                Beacon {
                    x: -825,
                    y: 492,
                    z: -355,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 685,
                    y: -540,
                    z: -346,
                },
                Beacon {
                    x: 558,
                    y: 649,
                    z: -782,
                },
                Beacon {
                    x: 345,
                    y: 770,
                    z: 731,
                },
                Beacon {
                    x: 580,
                    y: -565,
                    z: -397,
                },
                Beacon {
                    x: 711,
                    y: 724,
                    z: -790,
                },
                Beacon {
                    x: -476,
                    y: -596,
                    z: 899,
                },
                Beacon {
                    x: -512,
                    y: -840,
                    z: -508,
                },
                Beacon {
                    x: -568,
                    y: -746,
                    z: -468,
                },
                Beacon {
                    x: -831,
                    y: 530,
                    z: 708,
                },
                Beacon {
                    x: -706,
                    y: 553,
                    z: 714,
                },
                Beacon {
                    x: 403,
                    y: 759,
                    z: 565,
                },
                Beacon {
                    x: -774,
                    y: 431,
                    z: -320,
                },
                Beacon {
                    x: -80,
                    y: -103,
                    z: 76,
                },
                Beacon {
                    x: 893,
                    y: -743,
                    z: 494,
                },
                Beacon {
                    x: 633,
                    y: 843,
                    z: -781,
                },
                Beacon {
                    x: -460,
                    y: -856,
                    z: -547,
                },
                Beacon {
                    x: -856,
                    y: 498,
                    z: -396,
                },
                Beacon {
                    x: 728,
                    y: -762,
                    z: 426,
                },
                Beacon {
                    x: -466,
                    y: -757,
                    z: 819,
                },
                Beacon {
                    x: 45,
                    y: 34,
                    z: 148,
                },
                Beacon {
                    x: 551,
                    y: -495,
                    z: -348,
                },
                Beacon {
                    x: 784,
                    y: -817,
                    z: 575,
                },
                Beacon {
                    x: -813,
                    y: 599,
                    z: 804,
                },
                Beacon {
                    x: -508,
                    y: -541,
                    z: 838,
                },
                Beacon {
                    x: 399,
                    y: 785,
                    z: 683,
                },
                Beacon {
                    x: -812,
                    y: 296,
                    z: -441,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -833,
                    y: 419,
                    z: -685,
                },
                Beacon {
                    x: -649,
                    y: 471,
                    z: 788,
                },
                Beacon {
                    x: 714,
                    y: -553,
                    z: 870,
                },
                Beacon {
                    x: -504,
                    y: -456,
                    z: -348,
                },
                Beacon {
                    x: 747,
                    y: 564,
                    z: -709,
                },
                Beacon {
                    x: -448,
                    y: -384,
                    z: -461,
                },
                Beacon {
                    x: 769,
                    y: -578,
                    z: 908,
                },
                Beacon {
                    x: 17,
                    y: 31,
                    z: -46,
                },
                Beacon {
                    x: -79,
                    y: -61,
                    z: 101,
                },
                Beacon {
                    x: -684,
                    y: 378,
                    z: 885,
                },
                Beacon {
                    x: 703,
                    y: 552,
                    z: -696,
                },
                Beacon {
                    x: -542,
                    y: -357,
                    z: -380,
                },
                Beacon {
                    x: -581,
                    y: 316,
                    z: 828,
                },
                Beacon {
                    x: -372,
                    y: -905,
                    z: 779,
                },
                Beacon {
                    x: 668,
                    y: 834,
                    z: 803,
                },
                Beacon {
                    x: 424,
                    y: -502,
                    z: -633,
                },
                Beacon {
                    x: -899,
                    y: 539,
                    z: -595,
                },
                Beacon {
                    x: -557,
                    y: -833,
                    z: 738,
                },
                Beacon {
                    x: -703,
                    y: 510,
                    z: -662,
                },
                Beacon {
                    x: 732,
                    y: 398,
                    z: -707,
                },
                Beacon {
                    x: 620,
                    y: 658,
                    z: 854,
                },
                Beacon {
                    x: 588,
                    y: -445,
                    z: -582,
                },
                Beacon {
                    x: 581,
                    y: 649,
                    z: 821,
                },
                Beacon {
                    x: -418,
                    y: -886,
                    z: 759,
                },
                Beacon {
                    x: 582,
                    y: -618,
                    z: -633,
                },
                Beacon {
                    x: 719,
                    y: -443,
                    z: 894,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 519,
                    y: -699,
                    z: -930,
                },
                Beacon {
                    x: 473,
                    y: -410,
                    z: 415,
                },
                Beacon {
                    x: 756,
                    y: 791,
                    z: 465,
                },
                Beacon {
                    x: 508,
                    y: -703,
                    z: -859,
                },
                Beacon {
                    x: 303,
                    y: -505,
                    z: 389,
                },
                Beacon {
                    x: 767,
                    y: 375,
                    z: -681,
                },
                Beacon {
                    x: 785,
                    y: 682,
                    z: 560,
                },
                Beacon {
                    x: -456,
                    y: -746,
                    z: -747,
                },
                Beacon {
                    x: 786,
                    y: 376,
                    z: -521,
                },
                Beacon {
                    x: -57,
                    y: -17,
                    z: -26,
                },
                Beacon {
                    x: 714,
                    y: 466,
                    z: -556,
                },
                Beacon {
                    x: -582,
                    y: 370,
                    z: -797,
                },
                Beacon {
                    x: 534,
                    y: -699,
                    z: -835,
                },
                Beacon {
                    x: -615,
                    y: 345,
                    z: 513,
                },
                Beacon {
                    x: -484,
                    y: -514,
                    z: 665,
                },
                Beacon {
                    x: 327,
                    y: -353,
                    z: 470,
                },
                Beacon {
                    x: 688,
                    y: 787,
                    z: 563,
                },
                Beacon {
                    x: -616,
                    y: -646,
                    z: -732,
                },
                Beacon {
                    x: -601,
                    y: -555,
                    z: 739,
                },
                Beacon {
                    x: -562,
                    y: 446,
                    z: 534,
                },
                Beacon {
                    x: -488,
                    y: -513,
                    z: 665,
                },
                Beacon {
                    x: -663,
                    y: 429,
                    z: -808,
                },
                Beacon {
                    x: -466,
                    y: 387,
                    z: 609,
                },
                Beacon {
                    x: -536,
                    y: -596,
                    z: -808,
                },
                Beacon {
                    x: -660,
                    y: 390,
                    z: -728,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 753,
                    y: -452,
                    z: -462,
                },
                Beacon {
                    x: -723,
                    y: 856,
                    z: 853,
                },
                Beacon {
                    x: -610,
                    y: -562,
                    z: 812,
                },
                Beacon {
                    x: -575,
                    y: -845,
                    z: -624,
                },
                Beacon {
                    x: 448,
                    y: -380,
                    z: 520,
                },
                Beacon {
                    x: -111,
                    y: 116,
                    z: 120,
                },
                Beacon {
                    x: 722,
                    y: 809,
                    z: 544,
                },
                Beacon {
                    x: -761,
                    y: 754,
                    z: 750,
                },
                Beacon {
                    x: -616,
                    y: -748,
                    z: -688,
                },
                Beacon {
                    x: 368,
                    y: 897,
                    z: -293,
                },
                Beacon {
                    x: 436,
                    y: -388,
                    z: 542,
                },
                Beacon {
                    x: -781,
                    y: -625,
                    z: 754,
                },
                Beacon {
                    x: -442,
                    y: -784,
                    z: -655,
                },
                Beacon {
                    x: -664,
                    y: 755,
                    z: 842,
                },
                Beacon {
                    x: 741,
                    y: -327,
                    z: -621,
                },
                Beacon {
                    x: -631,
                    y: 777,
                    z: -751,
                },
                Beacon {
                    x: 800,
                    y: 778,
                    z: 643,
                },
                Beacon {
                    x: 277,
                    y: 921,
                    z: -445,
                },
                Beacon {
                    x: -13,
                    y: -48,
                    z: -24,
                },
                Beacon {
                    x: 762,
                    y: 676,
                    z: 667,
                },
                Beacon {
                    x: -770,
                    y: -446,
                    z: 803,
                },
                Beacon {
                    x: -615,
                    y: 708,
                    z: -541,
                },
                Beacon {
                    x: 368,
                    y: 880,
                    z: -372,
                },
                Beacon {
                    x: 580,
                    y: -385,
                    z: 418,
                },
                Beacon {
                    x: -580,
                    y: 840,
                    z: -650,
                },
                Beacon {
                    x: 697,
                    y: -480,
                    z: -599,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 794,
                    y: 761,
                    z: 517,
                },
                Beacon {
                    x: -510,
                    y: -616,
                    z: 596,
                },
                Beacon {
                    x: -515,
                    y: 569,
                    z: -319,
                },
                Beacon {
                    x: 456,
                    y: -763,
                    z: -317,
                },
                Beacon {
                    x: -754,
                    y: -643,
                    z: 617,
                },
                Beacon {
                    x: 730,
                    y: 444,
                    z: -487,
                },
                Beacon {
                    x: -461,
                    y: -511,
                    z: -373,
                },
                Beacon {
                    x: -422,
                    y: 459,
                    z: 511,
                },
                Beacon {
                    x: 702,
                    y: 500,
                    z: -624,
                },
                Beacon {
                    x: -623,
                    y: -582,
                    z: 594,
                },
                Beacon {
                    x: -38,
                    y: 122,
                    z: -61,
                },
                Beacon {
                    x: -346,
                    y: 423,
                    z: 614,
                },
                Beacon {
                    x: 802,
                    y: 749,
                    z: 495,
                },
                Beacon {
                    x: 855,
                    y: 456,
                    z: -646,
                },
                Beacon {
                    x: -431,
                    y: 625,
                    z: -388,
                },
                Beacon {
                    x: 378,
                    y: -736,
                    z: 739,
                },
                Beacon {
                    x: -370,
                    y: 654,
                    z: -281,
                },
                Beacon {
                    x: 0,
                    y: -28,
                    z: 53,
                },
                Beacon {
                    x: 532,
                    y: -752,
                    z: -493,
                },
                Beacon {
                    x: -469,
                    y: -422,
                    z: -307,
                },
                Beacon {
                    x: -349,
                    y: -431,
                    z: -349,
                },
                Beacon {
                    x: 524,
                    y: -661,
                    z: 778,
                },
                Beacon {
                    x: -471,
                    y: 389,
                    z: 477,
                },
                Beacon {
                    x: 797,
                    y: 617,
                    z: 393,
                },
                Beacon {
                    x: 556,
                    y: -696,
                    z: 702,
                },
                Beacon {
                    x: 437,
                    y: -733,
                    z: -310,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 366,
                    y: -727,
                    z: -920,
                },
                Beacon {
                    x: -607,
                    y: -861,
                    z: 320,
                },
                Beacon {
                    x: -84,
                    y: -90,
                    z: -148,
                },
                Beacon {
                    x: 310,
                    y: 389,
                    z: 243,
                },
                Beacon {
                    x: -805,
                    y: -924,
                    z: -746,
                },
                Beacon {
                    x: 439,
                    y: -652,
                    z: -878,
                },
                Beacon {
                    x: -668,
                    y: -900,
                    z: 513,
                },
                Beacon {
                    x: -877,
                    y: -832,
                    z: -832,
                },
                Beacon {
                    x: 402,
                    y: 330,
                    z: 267,
                },
                Beacon { x: -23, y: 4, z: 0 },
                Beacon {
                    x: -685,
                    y: 394,
                    z: 487,
                },
                Beacon {
                    x: 703,
                    y: 561,
                    z: -694,
                },
                Beacon {
                    x: -577,
                    y: 399,
                    z: 518,
                },
                Beacon {
                    x: 799,
                    y: -373,
                    z: 599,
                },
                Beacon {
                    x: 643,
                    y: -357,
                    z: 590,
                },
                Beacon {
                    x: 700,
                    y: -424,
                    z: 466,
                },
                Beacon {
                    x: -664,
                    y: -911,
                    z: 333,
                },
                Beacon {
                    x: -491,
                    y: 443,
                    z: 523,
                },
                Beacon {
                    x: 527,
                    y: 516,
                    z: -760,
                },
                Beacon {
                    x: -810,
                    y: 749,
                    z: -836,
                },
                Beacon {
                    x: 343,
                    y: -716,
                    z: -813,
                },
                Beacon {
                    x: -774,
                    y: -911,
                    z: -757,
                },
                Beacon {
                    x: -663,
                    y: 718,
                    z: -874,
                },
                Beacon {
                    x: -671,
                    y: 743,
                    z: -964,
                },
                Beacon {
                    x: 575,
                    y: 402,
                    z: 262,
                },
                Beacon {
                    x: 574,
                    y: 586,
                    z: -607,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -610,
                    y: 939,
                    z: -577,
                },
                Beacon {
                    x: 745,
                    y: -437,
                    z: -467,
                },
                Beacon {
                    x: 632,
                    y: -471,
                    z: -465,
                },
                Beacon {
                    x: -418,
                    y: 974,
                    z: -569,
                },
                Beacon {
                    x: 477,
                    y: 665,
                    z: 864,
                },
                Beacon {
                    x: -725,
                    y: -543,
                    z: -256,
                },
                Beacon {
                    x: 565,
                    y: 739,
                    z: 908,
                },
                Beacon {
                    x: -580,
                    y: -284,
                    z: 615,
                },
                Beacon {
                    x: -536,
                    y: -468,
                    z: 632,
                },
                Beacon {
                    x: -538,
                    y: -376,
                    z: 677,
                },
                Beacon {
                    x: 786,
                    y: -631,
                    z: 436,
                },
                Beacon {
                    x: 906,
                    y: -622,
                    z: 478,
                },
                Beacon {
                    x: 501,
                    y: -410,
                    z: -488,
                },
                Beacon {
                    x: 782,
                    y: 432,
                    z: -465,
                },
                Beacon {
                    x: 15,
                    y: -10,
                    z: 12,
                },
                Beacon {
                    x: -587,
                    y: 915,
                    z: -618,
                },
                Beacon {
                    x: 581,
                    y: 408,
                    z: -397,
                },
                Beacon {
                    x: 602,
                    y: 495,
                    z: -472,
                },
                Beacon {
                    x: -555,
                    y: 463,
                    z: 781,
                },
                Beacon {
                    x: -675,
                    y: -401,
                    z: -255,
                },
                Beacon {
                    x: -646,
                    y: -516,
                    z: -360,
                },
                Beacon {
                    x: -490,
                    y: 538,
                    z: 819,
                },
                Beacon {
                    x: 794,
                    y: -542,
                    z: 502,
                },
                Beacon {
                    x: -642,
                    y: 505,
                    z: 767,
                },
                Beacon {
                    x: 85,
                    y: 168,
                    z: 82,
                },
                Beacon {
                    x: 413,
                    y: 726,
                    z: 832,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -866,
                    y: -431,
                    z: 363,
                },
                Beacon {
                    x: -787,
                    y: -475,
                    z: 408,
                },
                Beacon {
                    x: 748,
                    y: -807,
                    z: 308,
                },
                Beacon {
                    x: 520,
                    y: 388,
                    z: -742,
                },
                Beacon {
                    x: -843,
                    y: -505,
                    z: -682,
                },
                Beacon {
                    x: 632,
                    y: -817,
                    z: 270,
                },
                Beacon {
                    x: -550,
                    y: 662,
                    z: 747,
                },
                Beacon {
                    x: 378,
                    y: -510,
                    z: -821,
                },
                Beacon {
                    x: -812,
                    y: 850,
                    z: -524,
                },
                Beacon {
                    x: 542,
                    y: 773,
                    z: 472,
                },
                Beacon {
                    x: 513,
                    y: 822,
                    z: 402,
                },
                Beacon {
                    x: -850,
                    y: 729,
                    z: -389,
                },
                Beacon {
                    x: 411,
                    y: -331,
                    z: -783,
                },
                Beacon {
                    x: 673,
                    y: 380,
                    z: -817,
                },
                Beacon {
                    x: -861,
                    y: -464,
                    z: -766,
                },
                Beacon {
                    x: -498,
                    y: 623,
                    z: 763,
                },
                Beacon {
                    x: 278,
                    y: -419,
                    z: -835,
                },
                Beacon {
                    x: 647,
                    y: 824,
                    z: 447,
                },
                Beacon {
                    x: -42,
                    y: -30,
                    z: -33,
                },
                Beacon {
                    x: -121,
                    y: 127,
                    z: -177,
                },
                Beacon {
                    x: -908,
                    y: -389,
                    z: -706,
                },
                Beacon {
                    x: -722,
                    y: -327,
                    z: 348,
                },
                Beacon {
                    x: 698,
                    y: -841,
                    z: 447,
                },
                Beacon {
                    x: -894,
                    y: 774,
                    z: -591,
                },
                Beacon {
                    x: -606,
                    y: 702,
                    z: 653,
                },
                Beacon {
                    x: 595,
                    y: 389,
                    z: -936,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 603,
                    y: -885,
                    z: 752,
                },
                Beacon {
                    x: -493,
                    y: -637,
                    z: -640,
                },
                Beacon {
                    x: 649,
                    y: -808,
                    z: 797,
                },
                Beacon {
                    x: 758,
                    y: 447,
                    z: -468,
                },
                Beacon {
                    x: -765,
                    y: -836,
                    z: 358,
                },
                Beacon {
                    x: -546,
                    y: 737,
                    z: -807,
                },
                Beacon {
                    x: -723,
                    y: -913,
                    z: 382,
                },
                Beacon {
                    x: 581,
                    y: 478,
                    z: -445,
                },
                Beacon {
                    x: 469,
                    y: 604,
                    z: 509,
                },
                Beacon {
                    x: -598,
                    y: 467,
                    z: 457,
                },
                Beacon {
                    x: 649,
                    y: 665,
                    z: 474,
                },
                Beacon {
                    x: 637,
                    y: 509,
                    z: -516,
                },
                Beacon {
                    x: -572,
                    y: 572,
                    z: 488,
                },
                Beacon {
                    x: -133,
                    y: 53,
                    z: -44,
                },
                Beacon {
                    x: 330,
                    y: -417,
                    z: -364,
                },
                Beacon {
                    x: 426,
                    y: -474,
                    z: -374,
                },
                Beacon {
                    x: -805,
                    y: -830,
                    z: 405,
                },
                Beacon {
                    x: -593,
                    y: -480,
                    z: -618,
                },
                Beacon {
                    x: 669,
                    y: 651,
                    z: 487,
                },
                Beacon {
                    x: 327,
                    y: -644,
                    z: -357,
                },
                Beacon {
                    x: 569,
                    y: -750,
                    z: 780,
                },
                Beacon {
                    x: -673,
                    y: 523,
                    z: 584,
                },
                Beacon {
                    x: -37,
                    y: -131,
                    z: -22,
                },
                Beacon {
                    x: -575,
                    y: -580,
                    z: -662,
                },
                Beacon {
                    x: -539,
                    y: 730,
                    z: -795,
                },
                Beacon {
                    x: -619,
                    y: 620,
                    z: -834,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -630,
                    y: -522,
                    z: -566,
                },
                Beacon {
                    x: -572,
                    y: -620,
                    z: -567,
                },
                Beacon {
                    x: -363,
                    y: 683,
                    z: -472,
                },
                Beacon {
                    x: 745,
                    y: 472,
                    z: 545,
                },
                Beacon {
                    x: 655,
                    y: -429,
                    z: 473,
                },
                Beacon {
                    x: 740,
                    y: 475,
                    z: 367,
                },
                Beacon {
                    x: 766,
                    y: 520,
                    z: 420,
                },
                Beacon {
                    x: 824,
                    y: 517,
                    z: -575,
                },
                Beacon {
                    x: -315,
                    y: 692,
                    z: -446,
                },
                Beacon {
                    x: 893,
                    y: -842,
                    z: -518,
                },
                Beacon {
                    x: 773,
                    y: -815,
                    z: -558,
                },
                Beacon {
                    x: -565,
                    y: -530,
                    z: 714,
                },
                Beacon {
                    x: 911,
                    y: -849,
                    z: -534,
                },
                Beacon {
                    x: 703,
                    y: 430,
                    z: -496,
                },
                Beacon {
                    x: -479,
                    y: -625,
                    z: 780,
                },
                Beacon {
                    x: -614,
                    y: -662,
                    z: 722,
                },
                Beacon {
                    x: -408,
                    y: 577,
                    z: -518,
                },
                Beacon {
                    x: 122,
                    y: -47,
                    z: 4,
                },
                Beacon {
                    x: -451,
                    y: 718,
                    z: 408,
                },
                Beacon {
                    x: -343,
                    y: 695,
                    z: 393,
                },
                Beacon {
                    x: -564,
                    y: -689,
                    z: -530,
                },
                Beacon {
                    x: 564,
                    y: -509,
                    z: 576,
                },
                Beacon {
                    x: 666,
                    y: 517,
                    z: -531,
                },
                Beacon {
                    x: -403,
                    y: 709,
                    z: 578,
                },
                Beacon {
                    x: 674,
                    y: -393,
                    z: 553,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -646,
                    y: 523,
                    z: -341,
                },
                Beacon {
                    x: 688,
                    y: -727,
                    z: 903,
                },
                Beacon {
                    x: 823,
                    y: -765,
                    z: 782,
                },
                Beacon {
                    x: -435,
                    y: -506,
                    z: 735,
                },
                Beacon {
                    x: 429,
                    y: 496,
                    z: -278,
                },
                Beacon {
                    x: 506,
                    y: -823,
                    z: -336,
                },
                Beacon {
                    x: 391,
                    y: 463,
                    z: -457,
                },
                Beacon {
                    x: -97,
                    y: 6,
                    z: 55,
                },
                Beacon {
                    x: 395,
                    y: 758,
                    z: 433,
                },
                Beacon {
                    x: -517,
                    y: -395,
                    z: -394,
                },
                Beacon {
                    x: -390,
                    y: -369,
                    z: 762,
                },
                Beacon {
                    x: -625,
                    y: 430,
                    z: -275,
                },
                Beacon {
                    x: -430,
                    y: -393,
                    z: 892,
                },
                Beacon {
                    x: -702,
                    y: 705,
                    z: 679,
                },
                Beacon {
                    x: 377,
                    y: 363,
                    z: -366,
                },
                Beacon {
                    x: -749,
                    y: 552,
                    z: 749,
                },
                Beacon {
                    x: 530,
                    y: -758,
                    z: -452,
                },
                Beacon {
                    x: -566,
                    y: -324,
                    z: -446,
                },
                Beacon {
                    x: -741,
                    y: 406,
                    z: -352,
                },
                Beacon {
                    x: 276,
                    y: 680,
                    z: 442,
                },
                Beacon {
                    x: -573,
                    y: 618,
                    z: 704,
                },
                Beacon {
                    x: 621,
                    y: -809,
                    z: 827,
                },
                Beacon {
                    x: 470,
                    y: 605,
                    z: 441,
                },
                Beacon {
                    x: 446,
                    y: -776,
                    z: -411,
                },
                Beacon {
                    x: -601,
                    y: -290,
                    z: -489,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 487,
                    y: 432,
                    z: -424,
                },
                Beacon {
                    x: -236,
                    y: 867,
                    z: 618,
                },
                Beacon {
                    x: 837,
                    y: 644,
                    z: 599,
                },
                Beacon {
                    x: 680,
                    y: 744,
                    z: 621,
                },
                Beacon {
                    x: 907,
                    y: -504,
                    z: -760,
                },
                Beacon {
                    x: -712,
                    y: -776,
                    z: -488,
                },
                Beacon {
                    x: -598,
                    y: 787,
                    z: -397,
                },
                Beacon {
                    x: 483,
                    y: -748,
                    z: 725,
                },
                Beacon {
                    x: -598,
                    y: 658,
                    z: -375,
                },
                Beacon {
                    x: -19,
                    y: -107,
                    z: -36,
                },
                Beacon {
                    x: -668,
                    y: -881,
                    z: -396,
                },
                Beacon {
                    x: -675,
                    y: -625,
                    z: 373,
                },
                Beacon {
                    x: -267,
                    y: 749,
                    z: 733,
                },
                Beacon {
                    x: -706,
                    y: -724,
                    z: 451,
                },
                Beacon {
                    x: 737,
                    y: 662,
                    z: 643,
                },
                Beacon {
                    x: 404,
                    y: 514,
                    z: -401,
                },
                Beacon {
                    x: 761,
                    y: -465,
                    z: -685,
                },
                Beacon {
                    x: -737,
                    y: -872,
                    z: -396,
                },
                Beacon {
                    x: 481,
                    y: 472,
                    z: -377,
                },
                Beacon {
                    x: 488,
                    y: -850,
                    z: 593,
                },
                Beacon {
                    x: 491,
                    y: -726,
                    z: 737,
                },
                Beacon {
                    x: 176,
                    y: 48,
                    z: -56,
                },
                Beacon {
                    x: -568,
                    y: 719,
                    z: -542,
                },
                Beacon {
                    x: -322,
                    y: 820,
                    z: 603,
                },
                Beacon {
                    x: 729,
                    y: -541,
                    z: -720,
                },
                Beacon {
                    x: -573,
                    y: -601,
                    z: 411,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 837,
                    y: 652,
                    z: 469,
                },
                Beacon {
                    x: 404,
                    y: 570,
                    z: -869,
                },
                Beacon {
                    x: -495,
                    y: -798,
                    z: -461,
                },
                Beacon {
                    x: 395,
                    y: 492,
                    z: -750,
                },
                Beacon {
                    x: -340,
                    y: -624,
                    z: 357,
                },
                Beacon {
                    x: -285,
                    y: -494,
                    z: 428,
                },
                Beacon {
                    x: -495,
                    y: 787,
                    z: -754,
                },
                Beacon {
                    x: -826,
                    y: 467,
                    z: 514,
                },
                Beacon {
                    x: 882,
                    y: -882,
                    z: -390,
                },
                Beacon {
                    x: -566,
                    y: -878,
                    z: -462,
                },
                Beacon {
                    x: -254,
                    y: -609,
                    z: 310,
                },
                Beacon {
                    x: -37,
                    y: -30,
                    z: -61,
                },
                Beacon {
                    x: 719,
                    y: -584,
                    z: 515,
                },
                Beacon {
                    x: 825,
                    y: -872,
                    z: -486,
                },
                Beacon {
                    x: 788,
                    y: 623,
                    z: 539,
                },
                Beacon {
                    x: -560,
                    y: -654,
                    z: -425,
                },
                Beacon {
                    x: 655,
                    y: -889,
                    z: -387,
                },
                Beacon {
                    x: 839,
                    y: 518,
                    z: 514,
                },
                Beacon {
                    x: -404,
                    y: 887,
                    z: -768,
                },
                Beacon {
                    x: -792,
                    y: 417,
                    z: 375,
                },
                Beacon {
                    x: 403,
                    y: 645,
                    z: -675,
                },
                Beacon {
                    x: -316,
                    y: 754,
                    z: -716,
                },
                Beacon {
                    x: 153,
                    y: 58,
                    z: -85,
                },
                Beacon {
                    x: -802,
                    y: 533,
                    z: 337,
                },
                Beacon {
                    x: 819,
                    y: -628,
                    z: 533,
                },
                Beacon {
                    x: 662,
                    y: -606,
                    z: 433,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -365,
                    y: -825,
                    z: 601,
                },
                Beacon {
                    x: 579,
                    y: 616,
                    z: -693,
                },
                Beacon {
                    x: -372,
                    y: -492,
                    z: -561,
                },
                Beacon {
                    x: 536,
                    y: 799,
                    z: 546,
                },
                Beacon {
                    x: 750,
                    y: -509,
                    z: -791,
                },
                Beacon {
                    x: -699,
                    y: 666,
                    z: 813,
                },
                Beacon {
                    x: 690,
                    y: -436,
                    z: 476,
                },
                Beacon {
                    x: 483,
                    y: 598,
                    z: -835,
                },
                Beacon {
                    x: 905,
                    y: -499,
                    z: -832,
                },
                Beacon {
                    x: -736,
                    y: 614,
                    z: 845,
                },
                Beacon {
                    x: -390,
                    y: -307,
                    z: -648,
                },
                Beacon {
                    x: 498,
                    y: 727,
                    z: 605,
                },
                Beacon {
                    x: 436,
                    y: 705,
                    z: 501,
                },
                Beacon {
                    x: -405,
                    y: -670,
                    z: 669,
                },
                Beacon {
                    x: 755,
                    y: -540,
                    z: -818,
                },
                Beacon {
                    x: 864,
                    y: -469,
                    z: 440,
                },
                Beacon {
                    x: -716,
                    y: 899,
                    z: -772,
                },
                Beacon {
                    x: -21,
                    y: -19,
                    z: -31,
                },
                Beacon {
                    x: -658,
                    y: 826,
                    z: -848,
                },
                Beacon {
                    x: -791,
                    y: 479,
                    z: 796,
                },
                Beacon {
                    x: -604,
                    y: 824,
                    z: -678,
                },
                Beacon {
                    x: 731,
                    y: -500,
                    z: 379,
                },
                Beacon {
                    x: 112,
                    y: 113,
                    z: 87,
                },
                Beacon {
                    x: -308,
                    y: -763,
                    z: 704,
                },
                Beacon {
                    x: -339,
                    y: -483,
                    z: -600,
                },
                Beacon {
                    x: 513,
                    y: 564,
                    z: -805,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -648,
                    y: 877,
                    z: 692,
                },
                Beacon {
                    x: -641,
                    y: 681,
                    z: 616,
                },
                Beacon {
                    x: 928,
                    y: -622,
                    z: -787,
                },
                Beacon {
                    x: -314,
                    y: -397,
                    z: 632,
                },
                Beacon {
                    x: 685,
                    y: 442,
                    z: 468,
                },
                Beacon {
                    x: -426,
                    y: 540,
                    z: -629,
                },
                Beacon {
                    x: -421,
                    y: -487,
                    z: 586,
                },
                Beacon {
                    x: 143,
                    y: -24,
                    z: 99,
                },
                Beacon {
                    x: -651,
                    y: -302,
                    z: -563,
                },
                Beacon {
                    x: -619,
                    y: -363,
                    z: -508,
                },
                Beacon {
                    x: 576,
                    y: 545,
                    z: 520,
                },
                Beacon {
                    x: 468,
                    y: 581,
                    z: -569,
                },
                Beacon {
                    x: 585,
                    y: -499,
                    z: 346,
                },
                Beacon {
                    x: -661,
                    y: -382,
                    z: -724,
                },
                Beacon {
                    x: 669,
                    y: -485,
                    z: 510,
                },
                Beacon {
                    x: 908,
                    y: -680,
                    z: -835,
                },
                Beacon {
                    x: 950,
                    y: -793,
                    z: -735,
                },
                Beacon {
                    x: 606,
                    y: 490,
                    z: 596,
                },
                Beacon {
                    x: 648,
                    y: 518,
                    z: -598,
                },
                Beacon {
                    x: -647,
                    y: 730,
                    z: 821,
                },
                Beacon {
                    x: 637,
                    y: -411,
                    z: 403,
                },
                Beacon {
                    x: 587,
                    y: 571,
                    z: -644,
                },
                Beacon {
                    x: -452,
                    y: 404,
                    z: -617,
                },
                Beacon {
                    x: -496,
                    y: 510,
                    z: -590,
                },
                Beacon {
                    x: -296,
                    y: -493,
                    z: 563,
                },
                Beacon {
                    x: 42,
                    y: 131,
                    z: -41,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -796,
                    y: 669,
                    z: -655,
                },
                Beacon {
                    x: 664,
                    y: -571,
                    z: -614,
                },
                Beacon {
                    x: 753,
                    y: 635,
                    z: 526,
                },
                Beacon {
                    x: 810,
                    y: 530,
                    z: -606,
                },
                Beacon {
                    x: 536,
                    y: -264,
                    z: 411,
                },
                Beacon {
                    x: -84,
                    y: 142,
                    z: 41,
                },
                Beacon {
                    x: 511,
                    y: -501,
                    z: -567,
                },
                Beacon {
                    x: 458,
                    y: -255,
                    z: 530,
                },
                Beacon {
                    x: 23,
                    y: -11,
                    z: 125,
                },
                Beacon {
                    x: -363,
                    y: -757,
                    z: -739,
                },
                Beacon {
                    x: 611,
                    y: -263,
                    z: 605,
                },
                Beacon {
                    x: 786,
                    y: 453,
                    z: -490,
                },
                Beacon {
                    x: 853,
                    y: 544,
                    z: 512,
                },
                Beacon {
                    x: 720,
                    y: 456,
                    z: 564,
                },
                Beacon {
                    x: -783,
                    y: 578,
                    z: -588,
                },
                Beacon {
                    x: -339,
                    y: -698,
                    z: -831,
                },
                Beacon {
                    x: -832,
                    y: 635,
                    z: 819,
                },
                Beacon {
                    x: -916,
                    y: -742,
                    z: 890,
                },
                Beacon {
                    x: 693,
                    y: 519,
                    z: -511,
                },
                Beacon {
                    x: -912,
                    y: 706,
                    z: 818,
                },
                Beacon {
                    x: -906,
                    y: -685,
                    z: 794,
                },
                Beacon {
                    x: -923,
                    y: -515,
                    z: 893,
                },
                Beacon {
                    x: -688,
                    y: 700,
                    z: 855,
                },
                Beacon {
                    x: 598,
                    y: -526,
                    z: -438,
                },
                Beacon {
                    x: -866,
                    y: 578,
                    z: -625,
                },
                Beacon {
                    x: -358,
                    y: -685,
                    z: -566,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 787,
                    y: -541,
                    z: -764,
                },
                Beacon {
                    x: 580,
                    y: 742,
                    z: -302,
                },
                Beacon {
                    x: -594,
                    y: 689,
                    z: -647,
                },
                Beacon {
                    x: -403,
                    y: 709,
                    z: 600,
                },
                Beacon {
                    x: 732,
                    y: -510,
                    z: -731,
                },
                Beacon {
                    x: -355,
                    y: 837,
                    z: 679,
                },
                Beacon {
                    x: 17,
                    y: 104,
                    z: 76,
                },
                Beacon {
                    x: -764,
                    y: -444,
                    z: -614,
                },
                Beacon {
                    x: -762,
                    y: -446,
                    z: -432,
                },
                Beacon {
                    x: 586,
                    y: 959,
                    z: 579,
                },
                Beacon {
                    x: 572,
                    y: 795,
                    z: 663,
                },
                Beacon {
                    x: -807,
                    y: -759,
                    z: 760,
                },
                Beacon {
                    x: -458,
                    y: 785,
                    z: 570,
                },
                Beacon {
                    x: 396,
                    y: -490,
                    z: 823,
                },
                Beacon {
                    x: 574,
                    y: 719,
                    z: -427,
                },
                Beacon {
                    x: -643,
                    y: 484,
                    z: -653,
                },
                Beacon {
                    x: -863,
                    y: -745,
                    z: 773,
                },
                Beacon {
                    x: -790,
                    y: 613,
                    z: -654,
                },
                Beacon {
                    x: -836,
                    y: -481,
                    z: -513,
                },
                Beacon {
                    x: 444,
                    y: -391,
                    z: 878,
                },
                Beacon {
                    x: 498,
                    y: 902,
                    z: 631,
                },
                Beacon {
                    x: 484,
                    y: 673,
                    z: -320,
                },
                Beacon {
                    x: 712,
                    y: -535,
                    z: -757,
                },
                Beacon {
                    x: 346,
                    y: -517,
                    z: 837,
                },
                Beacon {
                    x: -858,
                    y: -738,
                    z: 905,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -723,
                    y: 676,
                    z: -443,
                },
                Beacon {
                    x: -767,
                    y: -524,
                    z: -615,
                },
                Beacon {
                    x: 661,
                    y: 495,
                    z: -893,
                },
                Beacon {
                    x: -803,
                    y: 786,
                    z: -482,
                },
                Beacon {
                    x: -849,
                    y: -581,
                    z: -637,
                },
                Beacon {
                    x: -796,
                    y: 793,
                    z: -470,
                },
                Beacon {
                    x: -491,
                    y: 517,
                    z: 388,
                },
                Beacon {
                    x: -27,
                    y: -4,
                    z: -61,
                },
                Beacon {
                    x: -768,
                    y: -404,
                    z: 487,
                },
                Beacon {
                    x: 524,
                    y: 370,
                    z: -859,
                },
                Beacon {
                    x: 605,
                    y: 715,
                    z: 688,
                },
                Beacon {
                    x: 621,
                    y: -770,
                    z: -521,
                },
                Beacon {
                    x: -172,
                    y: 101,
                    z: 24,
                },
                Beacon {
                    x: 627,
                    y: 446,
                    z: -823,
                },
                Beacon {
                    x: -546,
                    y: 368,
                    z: 444,
                },
                Beacon {
                    x: -813,
                    y: -406,
                    z: 621,
                },
                Beacon {
                    x: 615,
                    y: 808,
                    z: 735,
                },
                Beacon {
                    x: -749,
                    y: -424,
                    z: -659,
                },
                Beacon {
                    x: 360,
                    y: -712,
                    z: 685,
                },
                Beacon {
                    x: -565,
                    y: 436,
                    z: 279,
                },
                Beacon {
                    x: 641,
                    y: 720,
                    z: 858,
                },
                Beacon {
                    x: 745,
                    y: -773,
                    z: -665,
                },
                Beacon {
                    x: 351,
                    y: -596,
                    z: 525,
                },
                Beacon {
                    x: 409,
                    y: -533,
                    z: 696,
                },
                Beacon {
                    x: 643,
                    y: -778,
                    z: -533,
                },
                Beacon {
                    x: -743,
                    y: -455,
                    z: 592,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -63,
                    y: 169,
                    z: -79,
                },
                Beacon {
                    x: 526,
                    y: 762,
                    z: -393,
                },
                Beacon {
                    x: 885,
                    y: -267,
                    z: -724,
                },
                Beacon {
                    x: -691,
                    y: 831,
                    z: -698,
                },
                Beacon {
                    x: -783,
                    y: 570,
                    z: 443,
                },
                Beacon {
                    x: 442,
                    y: 706,
                    z: -462,
                },
                Beacon {
                    x: -326,
                    y: -441,
                    z: -723,
                },
                Beacon {
                    x: 482,
                    y: 942,
                    z: 403,
                },
                Beacon {
                    x: 21,
                    y: -5,
                    z: -6,
                },
                Beacon {
                    x: 357,
                    y: 908,
                    z: 540,
                },
                Beacon {
                    x: 381,
                    y: 677,
                    z: -355,
                },
                Beacon {
                    x: 795,
                    y: -686,
                    z: 449,
                },
                Beacon {
                    x: -719,
                    y: 887,
                    z: -561,
                },
                Beacon {
                    x: -423,
                    y: -482,
                    z: -787,
                },
                Beacon {
                    x: 702,
                    y: -639,
                    z: 459,
                },
                Beacon {
                    x: 844,
                    y: -422,
                    z: -714,
                },
                Beacon {
                    x: -725,
                    y: -636,
                    z: 704,
                },
                Beacon {
                    x: 707,
                    y: -809,
                    z: 423,
                },
                Beacon {
                    x: -786,
                    y: 714,
                    z: 319,
                },
                Beacon {
                    x: 433,
                    y: 872,
                    z: 437,
                },
                Beacon {
                    x: -791,
                    y: 582,
                    z: 421,
                },
                Beacon {
                    x: -546,
                    y: -647,
                    z: 655,
                },
                Beacon {
                    x: 871,
                    y: -417,
                    z: -683,
                },
                Beacon {
                    x: -844,
                    y: 875,
                    z: -720,
                },
                Beacon {
                    x: -609,
                    y: -476,
                    z: 713,
                },
                Beacon {
                    x: -387,
                    y: -308,
                    z: -754,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 762,
                    y: -884,
                    z: 815,
                },
                Beacon {
                    x: -593,
                    y: 480,
                    z: -449,
                },
                Beacon {
                    x: -694,
                    y: -877,
                    z: 840,
                },
                Beacon {
                    x: 738,
                    y: -622,
                    z: -717,
                },
                Beacon {
                    x: 320,
                    y: 333,
                    z: 517,
                },
                Beacon {
                    x: 496,
                    y: 344,
                    z: 593,
                },
                Beacon {
                    x: 653,
                    y: -873,
                    z: 937,
                },
                Beacon {
                    x: -775,
                    y: 435,
                    z: 585,
                },
                Beacon {
                    x: 313,
                    y: 363,
                    z: 498,
                },
                Beacon {
                    x: -736,
                    y: 360,
                    z: 439,
                },
                Beacon {
                    x: -699,
                    y: 400,
                    z: 617,
                },
                Beacon {
                    x: 310,
                    y: 674,
                    z: -408,
                },
                Beacon {
                    x: -583,
                    y: -889,
                    z: 704,
                },
                Beacon {
                    x: -614,
                    y: 453,
                    z: -693,
                },
                Beacon {
                    x: -127,
                    y: 29,
                    z: -13,
                },
                Beacon {
                    x: 706,
                    y: -683,
                    z: -781,
                },
                Beacon {
                    x: 782,
                    y: -696,
                    z: -629,
                },
                Beacon {
                    x: -591,
                    y: -389,
                    z: -412,
                },
                Beacon {
                    x: 745,
                    y: -834,
                    z: 774,
                },
                Beacon {
                    x: -591,
                    y: 514,
                    z: -562,
                },
                Beacon {
                    x: 34,
                    y: -142,
                    z: 138,
                },
                Beacon {
                    x: -523,
                    y: -365,
                    z: -613,
                },
                Beacon {
                    x: -592,
                    y: -780,
                    z: 775,
                },
                Beacon {
                    x: -598,
                    y: -452,
                    z: -592,
                },
                Beacon {
                    x: 352,
                    y: 665,
                    z: -288,
                },
                Beacon {
                    x: 376,
                    y: 585,
                    z: -400,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -639,
                    y: 516,
                    z: 498,
                },
                Beacon {
                    x: 559,
                    y: 607,
                    z: -498,
                },
                Beacon {
                    x: 545,
                    y: -703,
                    z: 602,
                },
                Beacon {
                    x: -625,
                    y: 375,
                    z: 619,
                },
                Beacon {
                    x: 601,
                    y: -628,
                    z: 475,
                },
                Beacon {
                    x: -623,
                    y: 376,
                    z: 593,
                },
                Beacon {
                    x: -972,
                    y: -755,
                    z: 740,
                },
                Beacon {
                    x: 331,
                    y: -576,
                    z: -789,
                },
                Beacon {
                    x: 496,
                    y: -602,
                    z: 506,
                },
                Beacon {
                    x: 448,
                    y: -649,
                    z: -816,
                },
                Beacon {
                    x: -136,
                    y: -1,
                    z: -77,
                },
                Beacon {
                    x: -596,
                    y: 561,
                    z: -524,
                },
                Beacon {
                    x: -629,
                    y: 491,
                    z: -454,
                },
                Beacon {
                    x: -917,
                    y: -896,
                    z: 773,
                },
                Beacon {
                    x: 403,
                    y: 619,
                    z: 492,
                },
                Beacon {
                    x: -40,
                    y: -132,
                    z: 22,
                },
                Beacon {
                    x: 590,
                    y: 551,
                    z: -427,
                },
                Beacon {
                    x: 621,
                    y: 728,
                    z: -450,
                },
                Beacon {
                    x: -687,
                    y: 478,
                    z: -488,
                },
                Beacon {
                    x: 464,
                    y: 439,
                    z: 527,
                },
                Beacon {
                    x: -574,
                    y: -716,
                    z: -611,
                },
                Beacon {
                    x: 417,
                    y: 639,
                    z: 505,
                },
                Beacon {
                    x: -575,
                    y: -609,
                    z: -726,
                },
                Beacon {
                    x: -948,
                    y: -818,
                    z: 591,
                },
                Beacon {
                    x: -471,
                    y: -603,
                    z: -554,
                },
                Beacon {
                    x: 338,
                    y: -569,
                    z: -777,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 706,
                    y: 540,
                    z: -887,
                },
                Beacon {
                    x: -593,
                    y: -459,
                    z: 281,
                },
                Beacon {
                    x: -658,
                    y: -739,
                    z: -830,
                },
                Beacon {
                    x: 375,
                    y: -435,
                    z: 378,
                },
                Beacon {
                    x: 409,
                    y: -296,
                    z: 328,
                },
                Beacon {
                    x: 641,
                    y: 504,
                    z: -743,
                },
                Beacon {
                    x: 885,
                    y: 795,
                    z: 665,
                },
                Beacon {
                    x: 399,
                    y: -258,
                    z: 444,
                },
                Beacon {
                    x: -566,
                    y: 590,
                    z: 733,
                },
                Beacon {
                    x: -784,
                    y: 658,
                    z: -798,
                },
                Beacon {
                    x: 149,
                    y: 165,
                    z: -81,
                },
                Beacon {
                    x: 797,
                    y: 847,
                    z: 798,
                },
                Beacon {
                    x: 706,
                    y: 833,
                    z: 698,
                },
                Beacon {
                    x: -733,
                    y: 667,
                    z: -790,
                },
                Beacon {
                    x: 635,
                    y: 541,
                    z: -967,
                },
                Beacon {
                    x: 861,
                    y: -278,
                    z: -713,
                },
                Beacon {
                    x: 916,
                    y: -349,
                    z: -583,
                },
                Beacon {
                    x: 947,
                    y: -231,
                    z: -698,
                },
                Beacon {
                    x: -456,
                    y: 443,
                    z: 770,
                },
                Beacon {
                    x: -821,
                    y: -742,
                    z: -790,
                },
                Beacon {
                    x: -633,
                    y: -559,
                    z: 267,
                },
                Beacon {
                    x: -721,
                    y: -792,
                    z: -864,
                },
                Beacon {
                    x: -773,
                    y: 560,
                    z: -788,
                },
                Beacon {
                    x: -599,
                    y: 514,
                    z: 741,
                },
                Beacon {
                    x: -613,
                    y: -385,
                    z: 346,
                },
                Beacon {
                    x: 15,
                    y: 25,
                    z: -160,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 570,
                    y: 694,
                    z: 717,
                },
                Beacon {
                    x: 585,
                    y: -959,
                    z: 574,
                },
                Beacon {
                    x: 769,
                    y: 732,
                    z: -671,
                },
                Beacon {
                    x: -734,
                    y: -514,
                    z: -771,
                },
                Beacon {
                    x: 698,
                    y: 742,
                    z: 638,
                },
                Beacon {
                    x: -877,
                    y: 239,
                    z: 412,
                },
                Beacon {
                    x: 422,
                    y: -608,
                    z: -684,
                },
                Beacon {
                    x: -847,
                    y: 485,
                    z: -712,
                },
                Beacon {
                    x: -25,
                    y: -142,
                    z: -16,
                },
                Beacon {
                    x: -401,
                    y: -929,
                    z: 501,
                },
                Beacon {
                    x: -787,
                    y: -673,
                    z: -699,
                },
                Beacon { x: 37, y: 6, z: 98 },
                Beacon {
                    x: -737,
                    y: -577,
                    z: -627,
                },
                Beacon {
                    x: 730,
                    y: 711,
                    z: -684,
                },
                Beacon {
                    x: -808,
                    y: 472,
                    z: -659,
                },
                Beacon {
                    x: 431,
                    y: -805,
                    z: -711,
                },
                Beacon {
                    x: 582,
                    y: -780,
                    z: 514,
                },
                Beacon {
                    x: -707,
                    y: 369,
                    z: -731,
                },
                Beacon {
                    x: 543,
                    y: -916,
                    z: 560,
                },
                Beacon {
                    x: -438,
                    y: -929,
                    z: 617,
                },
                Beacon {
                    x: 431,
                    y: -619,
                    z: -639,
                },
                Beacon {
                    x: 646,
                    y: 639,
                    z: 747,
                },
                Beacon {
                    x: -558,
                    y: -910,
                    z: 556,
                },
                Beacon {
                    x: -884,
                    y: 234,
                    z: 544,
                },
                Beacon {
                    x: -898,
                    y: 290,
                    z: 557,
                },
                Beacon {
                    x: 676,
                    y: 632,
                    z: -609,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -788,
                    y: 630,
                    z: -622,
                },
                Beacon {
                    x: 673,
                    y: 404,
                    z: 901,
                },
                Beacon {
                    x: -344,
                    y: -677,
                    z: 827,
                },
                Beacon {
                    x: 485,
                    y: -888,
                    z: -432,
                },
                Beacon {
                    x: 690,
                    y: -969,
                    z: 425,
                },
                Beacon {
                    x: 842,
                    y: -945,
                    z: 516,
                },
                Beacon {
                    x: -599,
                    y: -875,
                    z: -553,
                },
                Beacon {
                    x: -722,
                    y: -807,
                    z: -602,
                },
                Beacon {
                    x: 447,
                    y: 700,
                    z: -468,
                },
                Beacon {
                    x: -694,
                    y: 675,
                    z: -722,
                },
                Beacon {
                    x: -535,
                    y: -676,
                    z: 744,
                },
                Beacon {
                    x: -468,
                    y: -745,
                    z: 861,
                },
                Beacon {
                    x: -716,
                    y: 571,
                    z: -660,
                },
                Beacon {
                    x: 763,
                    y: -928,
                    z: 516,
                },
                Beacon {
                    x: -644,
                    y: 356,
                    z: 580,
                },
                Beacon {
                    x: 83,
                    y: -165,
                    z: 133,
                },
                Beacon {
                    x: 384,
                    y: -825,
                    z: -483,
                },
                Beacon {
                    x: -96,
                    y: -99,
                    z: 46,
                },
                Beacon {
                    x: 514,
                    y: 279,
                    z: 889,
                },
                Beacon {
                    x: 480,
                    y: 736,
                    z: -294,
                },
                Beacon {
                    x: 511,
                    y: 797,
                    z: -427,
                },
                Beacon {
                    x: -704,
                    y: 292,
                    z: 505,
                },
                Beacon {
                    x: 536,
                    y: 432,
                    z: 845,
                },
                Beacon {
                    x: 594,
                    y: -756,
                    z: -467,
                },
                Beacon {
                    x: -715,
                    y: -893,
                    z: -529,
                },
                Beacon {
                    x: 47,
                    y: 20,
                    z: -5,
                },
                Beacon {
                    x: -650,
                    y: 475,
                    z: 523,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -392,
                    y: 434,
                    z: -634,
                },
                Beacon {
                    x: 491,
                    y: -237,
                    z: 724,
                },
                Beacon {
                    x: -330,
                    y: 491,
                    z: -531,
                },
                Beacon {
                    x: -686,
                    y: -738,
                    z: -370,
                },
                Beacon {
                    x: -485,
                    y: 531,
                    z: -599,
                },
                Beacon {
                    x: -615,
                    y: -629,
                    z: 888,
                },
                Beacon {
                    x: -655,
                    y: -605,
                    z: -309,
                },
                Beacon {
                    x: -656,
                    y: -779,
                    z: 902,
                },
                Beacon {
                    x: 891,
                    y: 540,
                    z: -322,
                },
                Beacon {
                    x: -719,
                    y: -564,
                    z: -406,
                },
                Beacon {
                    x: -672,
                    y: 465,
                    z: 670,
                },
                Beacon {
                    x: 781,
                    y: -570,
                    z: -592,
                },
                Beacon {
                    x: 723,
                    y: -399,
                    z: -529,
                },
                Beacon {
                    x: 872,
                    y: 560,
                    z: -368,
                },
                Beacon {
                    x: -6,
                    y: 9,
                    z: -65,
                },
                Beacon {
                    x: 564,
                    y: 411,
                    z: 819,
                },
                Beacon {
                    x: 527,
                    y: -407,
                    z: 719,
                },
                Beacon {
                    x: -848,
                    y: 516,
                    z: 718,
                },
                Beacon {
                    x: -646,
                    y: -624,
                    z: 861,
                },
                Beacon {
                    x: 99,
                    y: 172,
                    z: -13,
                },
                Beacon {
                    x: 517,
                    y: -360,
                    z: 812,
                },
                Beacon {
                    x: -663,
                    y: 498,
                    z: 631,
                },
                Beacon {
                    x: 539,
                    y: 449,
                    z: 764,
                },
                Beacon {
                    x: 772,
                    y: -410,
                    z: -708,
                },
                Beacon {
                    x: 819,
                    y: 581,
                    z: -507,
                },
                Beacon {
                    x: 429,
                    y: 425,
                    z: 769,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -599,
                    y: 748,
                    z: -367,
                },
                Beacon {
                    x: -605,
                    y: 741,
                    z: -442,
                },
                Beacon {
                    x: 31,
                    y: -184,
                    z: 42,
                },
                Beacon {
                    x: 529,
                    y: 408,
                    z: -748,
                },
                Beacon {
                    x: 731,
                    y: -805,
                    z: 438,
                },
                Beacon {
                    x: -903,
                    y: 701,
                    z: 595,
                },
                Beacon {
                    x: -118,
                    y: -25,
                    z: -29,
                },
                Beacon {
                    x: 464,
                    y: 400,
                    z: 532,
                },
                Beacon {
                    x: -874,
                    y: -966,
                    z: 714,
                },
                Beacon {
                    x: 522,
                    y: 498,
                    z: 533,
                },
                Beacon {
                    x: 615,
                    y: 503,
                    z: -814,
                },
                Beacon {
                    x: -630,
                    y: 715,
                    z: -422,
                },
                Beacon {
                    x: -650,
                    y: -839,
                    z: -758,
                },
                Beacon {
                    x: 779,
                    y: -814,
                    z: 412,
                },
                Beacon {
                    x: 686,
                    y: -699,
                    z: 484,
                },
                Beacon {
                    x: 480,
                    y: 540,
                    z: -756,
                },
                Beacon {
                    x: -715,
                    y: -965,
                    z: 770,
                },
                Beacon {
                    x: -863,
                    y: 677,
                    z: 465,
                },
                Beacon {
                    x: -858,
                    y: 564,
                    z: 622,
                },
                Beacon {
                    x: 643,
                    y: -840,
                    z: -442,
                },
                Beacon {
                    x: 530,
                    y: 567,
                    z: 496,
                },
                Beacon {
                    x: -812,
                    y: -984,
                    z: 692,
                },
                Beacon {
                    x: -623,
                    y: -853,
                    z: -808,
                },
                Beacon {
                    x: 751,
                    y: -817,
                    z: -427,
                },
                Beacon {
                    x: -749,
                    y: -894,
                    z: -710,
                },
                Beacon {
                    x: 691,
                    y: -831,
                    z: -612,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 621,
                    y: 336,
                    z: -774,
                },
                Beacon {
                    x: 453,
                    y: 669,
                    z: 448,
                },
                Beacon {
                    x: 516,
                    y: 391,
                    z: -653,
                },
                Beacon {
                    x: 482,
                    y: -697,
                    z: -435,
                },
                Beacon {
                    x: -611,
                    y: 556,
                    z: -845,
                },
                Beacon {
                    x: -642,
                    y: 640,
                    z: -891,
                },
                Beacon {
                    x: 582,
                    y: 816,
                    z: 447,
                },
                Beacon {
                    x: 456,
                    y: -579,
                    z: -431,
                },
                Beacon {
                    x: -475,
                    y: 790,
                    z: 487,
                },
                Beacon {
                    x: -541,
                    y: -706,
                    z: 370,
                },
                Beacon {
                    x: 711,
                    y: -554,
                    z: 799,
                },
                Beacon {
                    x: -572,
                    y: -761,
                    z: 337,
                },
                Beacon {
                    x: 642,
                    y: -458,
                    z: 781,
                },
                Beacon {
                    x: 134,
                    y: -66,
                    z: 16,
                },
                Beacon {
                    x: 471,
                    y: -531,
                    z: -377,
                },
                Beacon {
                    x: -566,
                    y: -686,
                    z: 344,
                },
                Beacon {
                    x: -715,
                    y: -568,
                    z: -419,
                },
                Beacon {
                    x: -777,
                    y: 614,
                    z: -887,
                },
                Beacon {
                    x: -512,
                    y: 862,
                    z: 588,
                },
                Beacon {
                    x: -788,
                    y: -493,
                    z: -377,
                },
                Beacon {
                    x: -811,
                    y: -504,
                    z: -484,
                },
                Beacon {
                    x: 716,
                    y: 414,
                    z: -697,
                },
                Beacon {
                    x: 3,
                    y: 68,
                    z: -23,
                },
                Beacon {
                    x: 658,
                    y: 620,
                    z: 454,
                },
                Beacon {
                    x: 688,
                    y: -548,
                    z: 674,
                },
                Beacon {
                    x: -477,
                    y: 788,
                    z: 669,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 557,
                    y: 332,
                    z: 854,
                },
                Beacon {
                    x: -664,
                    y: -663,
                    z: -409,
                },
                Beacon {
                    x: 565,
                    y: -418,
                    z: -643,
                },
                Beacon {
                    x: 647,
                    y: -672,
                    z: 767,
                },
                Beacon {
                    x: -644,
                    y: 716,
                    z: -413,
                },
                Beacon {
                    x: 839,
                    y: 771,
                    z: -619,
                },
                Beacon {
                    x: -404,
                    y: 873,
                    z: 768,
                },
                Beacon {
                    x: 687,
                    y: 387,
                    z: 925,
                },
                Beacon {
                    x: -470,
                    y: 849,
                    z: 945,
                },
                Beacon {
                    x: 553,
                    y: -426,
                    z: -621,
                },
                Beacon {
                    x: -606,
                    y: 818,
                    z: -310,
                },
                Beacon {
                    x: -589,
                    y: -790,
                    z: 813,
                },
                Beacon {
                    x: 697,
                    y: -423,
                    z: -745,
                },
                Beacon {
                    x: 691,
                    y: -772,
                    z: 749,
                },
                Beacon {
                    x: -354,
                    y: 839,
                    z: 907,
                },
                Beacon {
                    x: 153,
                    y: 32,
                    z: 128,
                },
                Beacon {
                    x: 879,
                    y: 638,
                    z: -496,
                },
                Beacon {
                    x: 672,
                    y: 301,
                    z: 972,
                },
                Beacon {
                    x: 917,
                    y: 740,
                    z: -520,
                },
                Beacon {
                    x: 16,
                    y: -71,
                    z: 39,
                },
                Beacon {
                    x: -653,
                    y: -484,
                    z: -360,
                },
                Beacon {
                    x: 669,
                    y: -615,
                    z: 849,
                },
                Beacon {
                    x: -560,
                    y: -741,
                    z: 743,
                },
                Beacon {
                    x: -493,
                    y: -600,
                    z: -351,
                },
                Beacon {
                    x: -547,
                    y: 717,
                    z: -321,
                },
                Beacon {
                    x: -455,
                    y: -792,
                    z: 768,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 783,
                    y: 373,
                    z: 325,
                },
                Beacon {
                    x: 569,
                    y: 747,
                    z: -535,
                },
                Beacon {
                    x: -274,
                    y: -893,
                    z: -846,
                },
                Beacon {
                    x: -576,
                    y: 418,
                    z: -709,
                },
                Beacon {
                    x: 468,
                    y: -520,
                    z: -860,
                },
                Beacon {
                    x: -316,
                    y: -780,
                    z: -946,
                },
                Beacon {
                    x: -296,
                    y: -722,
                    z: -898,
                },
                Beacon {
                    x: -648,
                    y: 445,
                    z: 409,
                },
                Beacon {
                    x: -555,
                    y: -585,
                    z: 399,
                },
                Beacon {
                    x: -639,
                    y: -599,
                    z: 235,
                },
                Beacon {
                    x: -756,
                    y: 481,
                    z: -680,
                },
                Beacon {
                    x: 708,
                    y: 452,
                    z: 379,
                },
                Beacon {
                    x: -637,
                    y: 471,
                    z: -755,
                },
                Beacon {
                    x: 583,
                    y: 675,
                    z: -593,
                },
                Beacon {
                    x: 25,
                    y: -88,
                    z: -61,
                },
                Beacon {
                    x: 570,
                    y: 493,
                    z: -548,
                },
                Beacon {
                    x: 497,
                    y: -680,
                    z: 593,
                },
                Beacon {
                    x: -587,
                    y: -511,
                    z: 292,
                },
                Beacon {
                    x: 770,
                    y: 352,
                    z: 286,
                },
                Beacon {
                    x: -618,
                    y: 390,
                    z: 485,
                },
                Beacon {
                    x: 515,
                    y: -584,
                    z: -850,
                },
                Beacon {
                    x: 573,
                    y: -630,
                    z: 513,
                },
                Beacon {
                    x: 558,
                    y: -530,
                    z: 673,
                },
                Beacon {
                    x: -539,
                    y: 342,
                    z: 357,
                },
                Beacon {
                    x: 578,
                    y: -596,
                    z: -756,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -733,
                    y: 501,
                    z: 952,
                },
                Beacon {
                    x: -591,
                    y: -621,
                    z: 947,
                },
                Beacon {
                    x: 654,
                    y: -409,
                    z: 555,
                },
                Beacon {
                    x: 407,
                    y: 338,
                    z: 768,
                },
                Beacon {
                    x: 887,
                    y: -447,
                    z: -332,
                },
                Beacon {
                    x: -456,
                    y: -584,
                    z: 889,
                },
                Beacon {
                    x: -769,
                    y: 422,
                    z: -708,
                },
                Beacon {
                    x: 744,
                    y: 564,
                    z: -520,
                },
                Beacon {
                    x: -506,
                    y: -557,
                    z: -363,
                },
                Beacon {
                    x: 480,
                    y: 380,
                    z: 693,
                },
                Beacon {
                    x: -622,
                    y: 384,
                    z: -677,
                },
                Beacon {
                    x: -440,
                    y: -724,
                    z: -399,
                },
                Beacon {
                    x: 384,
                    y: 445,
                    z: 757,
                },
                Beacon {
                    x: 615,
                    y: 651,
                    z: -545,
                },
                Beacon {
                    x: 629,
                    y: -383,
                    z: 575,
                },
                Beacon {
                    x: 26,
                    y: 39,
                    z: 68,
                },
                Beacon {
                    x: -764,
                    y: 512,
                    z: 965,
                },
                Beacon {
                    x: 927,
                    y: -423,
                    z: -462,
                },
                Beacon {
                    x: 623,
                    y: -376,
                    z: 500,
                },
                Beacon {
                    x: 882,
                    y: -560,
                    z: -489,
                },
                Beacon {
                    x: -505,
                    y: -716,
                    z: 881,
                },
                Beacon {
                    x: -836,
                    y: 426,
                    z: 873,
                },
                Beacon {
                    x: 568,
                    y: 498,
                    z: -475,
                },
                Beacon {
                    x: -630,
                    y: 294,
                    z: -702,
                },
                Beacon {
                    x: -498,
                    y: -626,
                    z: -400,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -445,
                    y: 488,
                    z: 820,
                },
                Beacon {
                    x: -388,
                    y: -540,
                    z: 721,
                },
                Beacon {
                    x: 545,
                    y: -622,
                    z: 881,
                },
                Beacon {
                    x: 863,
                    y: 804,
                    z: -446,
                },
                Beacon {
                    x: 633,
                    y: 931,
                    z: 832,
                },
                Beacon {
                    x: -843,
                    y: -408,
                    z: -556,
                },
                Beacon {
                    x: 737,
                    y: -724,
                    z: -511,
                },
                Beacon {
                    x: -850,
                    y: -333,
                    z: -562,
                },
                Beacon {
                    x: 882,
                    y: 726,
                    z: -349,
                },
                Beacon {
                    x: 493,
                    y: -534,
                    z: 938,
                },
                Beacon {
                    x: -52,
                    y: -1,
                    z: 4,
                },
                Beacon {
                    x: 54,
                    y: 188,
                    z: -26,
                },
                Beacon {
                    x: -779,
                    y: 499,
                    z: -298,
                },
                Beacon {
                    x: -406,
                    y: -636,
                    z: 652,
                },
                Beacon {
                    x: -407,
                    y: 609,
                    z: 955,
                },
                Beacon {
                    x: -666,
                    y: 656,
                    z: -303,
                },
                Beacon {
                    x: 751,
                    y: -774,
                    z: -538,
                },
                Beacon {
                    x: -817,
                    y: -388,
                    z: -531,
                },
                Beacon {
                    x: 715,
                    y: 737,
                    z: 821,
                },
                Beacon {
                    x: -803,
                    y: 629,
                    z: -258,
                },
                Beacon {
                    x: 761,
                    y: 881,
                    z: 844,
                },
                Beacon {
                    x: 577,
                    y: -548,
                    z: 774,
                },
                Beacon {
                    x: -363,
                    y: 564,
                    z: 753,
                },
                Beacon {
                    x: -513,
                    y: -546,
                    z: 698,
                },
                Beacon {
                    x: 864,
                    y: 748,
                    z: -287,
                },
                Beacon {
                    x: 792,
                    y: -676,
                    z: -412,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: -598,
                    y: -573,
                    z: -714,
                },
                Beacon {
                    x: 679,
                    y: -407,
                    z: -450,
                },
                Beacon {
                    x: 769,
                    y: -782,
                    z: 513,
                },
                Beacon {
                    x: 794,
                    y: 672,
                    z: -515,
                },
                Beacon {
                    x: 690,
                    y: -543,
                    z: -306,
                },
                Beacon {
                    x: -729,
                    y: 841,
                    z: -584,
                },
                Beacon {
                    x: 799,
                    y: -600,
                    z: 451,
                },
                Beacon {
                    x: -74,
                    y: 117,
                    z: 181,
                },
                Beacon {
                    x: 902,
                    y: -730,
                    z: 472,
                },
                Beacon {
                    x: -481,
                    y: -680,
                    z: 732,
                },
                Beacon {
                    x: -475,
                    y: -622,
                    z: -646,
                },
                Beacon {
                    x: -409,
                    y: -642,
                    z: 791,
                },
                Beacon {
                    x: -547,
                    y: 503,
                    z: 636,
                },
                Beacon {
                    x: 832,
                    y: 883,
                    z: 920,
                },
                Beacon {
                    x: 685,
                    y: 586,
                    z: -471,
                },
                Beacon {
                    x: -553,
                    y: 560,
                    z: 517,
                },
                Beacon {
                    x: -853,
                    y: 807,
                    z: -516,
                },
                Beacon {
                    x: 740,
                    y: -468,
                    z: -306,
                },
                Beacon {
                    x: 725,
                    y: 946,
                    z: 826,
                },
                Beacon {
                    x: -493,
                    y: 485,
                    z: 453,
                },
                Beacon {
                    x: -482,
                    y: -549,
                    z: -732,
                },
                Beacon {
                    x: -503,
                    y: -742,
                    z: 836,
                },
                Beacon {
                    x: 731,
                    y: 750,
                    z: 858,
                },
                Beacon {
                    x: -662,
                    y: 724,
                    z: -515,
                },
                Beacon {
                    x: 813,
                    y: 684,
                    z: -540,
                },
                Beacon {
                    x: 93,
                    y: 49,
                    z: 73,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 506,
                    y: -491,
                    z: 519,
                },
                Beacon {
                    x: -829,
                    y: 637,
                    z: 754,
                },
                Beacon {
                    x: 500,
                    y: -460,
                    z: -732,
                },
                Beacon {
                    x: -851,
                    y: 467,
                    z: -882,
                },
                Beacon {
                    x: 758,
                    y: 481,
                    z: 315,
                },
                Beacon {
                    x: -818,
                    y: 471,
                    z: 696,
                },
                Beacon {
                    x: 446,
                    y: 750,
                    z: -555,
                },
                Beacon {
                    x: 400,
                    y: -617,
                    z: -710,
                },
                Beacon {
                    x: -519,
                    y: -454,
                    z: -976,
                },
                Beacon {
                    x: 436,
                    y: -442,
                    z: 548,
                },
                Beacon {
                    x: -674,
                    y: 498,
                    z: -905,
                },
                Beacon {
                    x: -423,
                    y: -397,
                    z: -986,
                },
                Beacon {
                    x: 482,
                    y: -560,
                    z: -688,
                },
                Beacon {
                    x: -902,
                    y: -875,
                    z: 466,
                },
                Beacon {
                    x: 481,
                    y: -440,
                    z: 414,
                },
                Beacon {
                    x: -3,
                    y: 33,
                    z: -5,
                },
                Beacon {
                    x: -708,
                    y: -876,
                    z: 541,
                },
                Beacon {
                    x: 684,
                    y: 400,
                    z: 206,
                },
                Beacon {
                    x: -783,
                    y: -884,
                    z: 388,
                },
                Beacon {
                    x: -730,
                    y: 529,
                    z: -834,
                },
                Beacon {
                    x: -74,
                    y: -97,
                    z: -162,
                },
                Beacon {
                    x: -844,
                    y: 589,
                    z: 700,
                },
                Beacon {
                    x: 526,
                    y: 640,
                    z: -516,
                },
                Beacon {
                    x: -420,
                    y: -624,
                    z: -993,
                },
                Beacon {
                    x: 703,
                    y: 332,
                    z: 371,
                },
                Beacon {
                    x: 453,
                    y: 757,
                    z: -543,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 34,
                    y: -23,
                    z: 16,
                },
                Beacon {
                    x: 339,
                    y: 576,
                    z: 633,
                },
                Beacon {
                    x: 684,
                    y: -552,
                    z: -812,
                },
                Beacon {
                    x: -777,
                    y: 575,
                    z: -749,
                },
                Beacon {
                    x: -958,
                    y: 608,
                    z: 616,
                },
                Beacon {
                    x: -89,
                    y: 81,
                    z: 100,
                },
                Beacon {
                    x: -656,
                    y: -290,
                    z: -658,
                },
                Beacon {
                    x: 321,
                    y: 535,
                    z: 717,
                },
                Beacon {
                    x: -565,
                    y: -285,
                    z: -851,
                },
                Beacon {
                    x: -793,
                    y: 506,
                    z: -610,
                },
                Beacon {
                    x: 370,
                    y: -779,
                    z: 475,
                },
                Beacon {
                    x: -951,
                    y: 674,
                    z: 434,
                },
                Beacon {
                    x: 395,
                    y: 546,
                    z: -457,
                },
                Beacon {
                    x: 576,
                    y: -631,
                    z: -702,
                },
                Beacon {
                    x: -900,
                    y: 668,
                    z: 667,
                },
                Beacon {
                    x: 460,
                    y: 398,
                    z: -397,
                },
                Beacon {
                    x: -667,
                    y: -295,
                    z: -799,
                },
                Beacon {
                    x: 332,
                    y: -658,
                    z: 340,
                },
                Beacon {
                    x: 632,
                    y: -591,
                    z: -796,
                },
                Beacon {
                    x: 316,
                    y: 502,
                    z: -412,
                },
                Beacon {
                    x: 288,
                    y: -703,
                    z: 542,
                },
                Beacon {
                    x: -790,
                    y: -530,
                    z: 474,
                },
                Beacon {
                    x: 274,
                    y: 460,
                    z: 657,
                },
                Beacon {
                    x: -833,
                    y: 470,
                    z: -815,
                },
                Beacon {
                    x: -708,
                    y: -336,
                    z: 463,
                },
                Beacon {
                    x: -836,
                    y: -386,
                    z: 451,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 554,
                    y: 768,
                    z: 437,
                },
                Beacon {
                    x: -555,
                    y: 513,
                    z: -625,
                },
                Beacon {
                    x: 455,
                    y: -560,
                    z: -499,
                },
                Beacon {
                    x: -541,
                    y: -921,
                    z: 835,
                },
                Beacon {
                    x: 720,
                    y: 826,
                    z: 426,
                },
                Beacon {
                    x: -793,
                    y: 613,
                    z: 547,
                },
                Beacon {
                    x: -530,
                    y: 600,
                    z: -496,
                },
                Beacon {
                    x: -299,
                    y: -361,
                    z: -797,
                },
                Beacon {
                    x: -371,
                    y: -904,
                    z: 736,
                },
                Beacon {
                    x: 404,
                    y: -693,
                    z: -400,
                },
                Beacon {
                    x: -314,
                    y: -914,
                    z: 832,
                },
                Beacon {
                    x: 672,
                    y: 772,
                    z: 411,
                },
                Beacon {
                    x: 550,
                    y: -810,
                    z: 404,
                },
                Beacon {
                    x: -792,
                    y: 538,
                    z: 353,
                },
                Beacon {
                    x: 683,
                    y: 784,
                    z: -760,
                },
                Beacon {
                    x: 105,
                    y: -64,
                    z: 1,
                },
                Beacon {
                    x: -485,
                    y: 666,
                    z: -672,
                },
                Beacon {
                    x: 581,
                    y: -833,
                    z: 581,
                },
                Beacon {
                    x: 388,
                    y: -684,
                    z: -556,
                },
                Beacon {
                    x: -358,
                    y: -337,
                    z: -889,
                },
                Beacon {
                    x: 612,
                    y: -762,
                    z: 525,
                },
                Beacon {
                    x: 747,
                    y: 719,
                    z: -856,
                },
                Beacon {
                    x: 758,
                    y: 826,
                    z: -833,
                },
                Beacon {
                    x: -801,
                    y: 460,
                    z: 472,
                },
                Beacon {
                    x: -371,
                    y: -356,
                    z: -624,
                },
            ],
        },
        Scanner {
            beacons: vec![
                Beacon {
                    x: 438,
                    y: 880,
                    z: -534,
                },
                Beacon {
                    x: 612,
                    y: 654,
                    z: 778,
                },
                Beacon {
                    x: -16,
                    y: 54,
                    z: 96,
                },
                Beacon {
                    x: 556,
                    y: -825,
                    z: 836,
                },
                Beacon {
                    x: -440,
                    y: 559,
                    z: 584,
                },
                Beacon {
                    x: -692,
                    y: -503,
                    z: -666,
                },
                Beacon {
                    x: 326,
                    y: -698,
                    z: -442,
                },
                Beacon {
                    x: -389,
                    y: 666,
                    z: -460,
                },
                Beacon {
                    x: -451,
                    y: -407,
                    z: 630,
                },
                Beacon {
                    x: 325,
                    y: -642,
                    z: -283,
                },
                Beacon {
                    x: 452,
                    y: 830,
                    z: -507,
                },
                Beacon {
                    x: -501,
                    y: 709,
                    z: 504,
                },
                Beacon {
                    x: -413,
                    y: 662,
                    z: -486,
                },
                Beacon {
                    x: 428,
                    y: -775,
                    z: 848,
                },
                Beacon {
                    x: -318,
                    y: 662,
                    z: -471,
                },
                Beacon {
                    x: 474,
                    y: -631,
                    z: 825,
                },
                Beacon {
                    x: -440,
                    y: -466,
                    z: 711,
                },
                Beacon {
                    x: -727,
                    y: -413,
                    z: -738,
                },
                Beacon {
                    x: -567,
                    y: -412,
                    z: -719,
                },
                Beacon {
                    x: 696,
                    y: 640,
                    z: 942,
                },
                Beacon {
                    x: -516,
                    y: 609,
                    z: 664,
                },
                Beacon {
                    x: 644,
                    y: 728,
                    z: 885,
                },
                Beacon {
                    x: 80,
                    y: -81,
                    z: -9,
                },
                Beacon {
                    x: -520,
                    y: -427,
                    z: 708,
                },
                Beacon {
                    x: 307,
                    y: -620,
                    z: -345,
                },
                Beacon {
                    x: 397,
                    y: 782,
                    z: -408,
                },
            ],
        },
    ];

    // let scanners = [
    //     Scanner {
    //         beacons: vec![
    //             Beacon {
    //                 x: 404,
    //                 y: -588,
    //                 z: -901,
    //             },
    //             Beacon {
    //                 x: 528,
    //                 y: -643,
    //                 z: 409,
    //             },
    //             Beacon {
    //                 x: -838,
    //                 y: 591,
    //                 z: 734,
    //             },
    //             Beacon {
    //                 x: 390,
    //                 y: -675,
    //                 z: -793,
    //             },
    //             Beacon {
    //                 x: -537,
    //                 y: -823,
    //                 z: -458,
    //             },
    //             Beacon {
    //                 x: -485,
    //                 y: -357,
    //                 z: 347,
    //             },
    //             Beacon {
    //                 x: -345,
    //                 y: -311,
    //                 z: 381,
    //             },
    //             Beacon {
    //                 x: -661,
    //                 y: -816,
    //                 z: -575,
    //             },
    //             Beacon {
    //                 x: -876,
    //                 y: 649,
    //                 z: 763,
    //             },
    //             Beacon {
    //                 x: -618,
    //                 y: -824,
    //                 z: -621,
    //             },
    //             Beacon {
    //                 x: 553,
    //                 y: 345,
    //                 z: -567,
    //             },
    //             Beacon {
    //                 x: 474,
    //                 y: 580,
    //                 z: 667,
    //             },
    //             Beacon {
    //                 x: -447,
    //                 y: -329,
    //                 z: 318,
    //             },
    //             Beacon {
    //                 x: -584,
    //                 y: 868,
    //                 z: -557,
    //             },
    //             Beacon {
    //                 x: 544,
    //                 y: -627,
    //                 z: -890,
    //             },
    //             Beacon {
    //                 x: 564,
    //                 y: 392,
    //                 z: -477,
    //             },
    //             Beacon {
    //                 x: 455,
    //                 y: 729,
    //                 z: 728,
    //             },
    //             Beacon {
    //                 x: -892,
    //                 y: 524,
    //                 z: 684,
    //             },
    //             Beacon {
    //                 x: -689,
    //                 y: 845,
    //                 z: -530,
    //             },
    //             Beacon {
    //                 x: 423,
    //                 y: -701,
    //                 z: 434,
    //             },
    //             Beacon {
    //                 x: 7,
    //                 y: -33,
    //                 z: -71,
    //             },
    //             Beacon {
    //                 x: 630,
    //                 y: 319,
    //                 z: -379,
    //             },
    //             Beacon {
    //                 x: 443,
    //                 y: 580,
    //                 z: 662,
    //             },
    //             Beacon {
    //                 x: -789,
    //                 y: 900,
    //                 z: -551,
    //             },
    //             Beacon {
    //                 x: 459,
    //                 y: -707,
    //                 z: 401,
    //             },
    //         ],
    //     },
    //     Scanner {
    //         beacons: vec![
    //             Beacon {
    //                 x: 686,
    //                 y: 422,
    //                 z: 578,
    //             },
    //             Beacon {
    //                 x: 605,
    //                 y: 423,
    //                 z: 415,
    //             },
    //             Beacon {
    //                 x: 515,
    //                 y: 917,
    //                 z: -361,
    //             },
    //             Beacon {
    //                 x: -336,
    //                 y: 658,
    //                 z: 858,
    //             },
    //             Beacon {
    //                 x: 95,
    //                 y: 138,
    //                 z: 22,
    //             },
    //             Beacon {
    //                 x: -476,
    //                 y: 619,
    //                 z: 847,
    //             },
    //             Beacon {
    //                 x: -340,
    //                 y: -569,
    //                 z: -846,
    //             },
    //             Beacon {
    //                 x: 567,
    //                 y: -361,
    //                 z: 727,
    //             },
    //             Beacon {
    //                 x: -460,
    //                 y: 603,
    //                 z: -452,
    //             },
    //             Beacon {
    //                 x: 669,
    //                 y: -402,
    //                 z: 600,
    //             },
    //             Beacon {
    //                 x: 729,
    //                 y: 430,
    //                 z: 532,
    //             },
    //             Beacon {
    //                 x: -500,
    //                 y: -761,
    //                 z: 534,
    //             },
    //             Beacon {
    //                 x: -322,
    //                 y: 571,
    //                 z: 750,
    //             },
    //             Beacon {
    //                 x: -466,
    //                 y: -666,
    //                 z: -811,
    //             },
    //             Beacon {
    //                 x: -429,
    //                 y: -592,
    //                 z: 574,
    //             },
    //             Beacon {
    //                 x: -355,
    //                 y: 545,
    //                 z: -477,
    //             },
    //             Beacon {
    //                 x: 703,
    //                 y: -491,
    //                 z: -529,
    //             },
    //             Beacon {
    //                 x: -328,
    //                 y: -685,
    //                 z: 520,
    //             },
    //             Beacon {
    //                 x: 413,
    //                 y: 935,
    //                 z: -424,
    //             },
    //             Beacon {
    //                 x: -391,
    //                 y: 539,
    //                 z: -444,
    //             },
    //             Beacon {
    //                 x: 586,
    //                 y: -435,
    //                 z: 557,
    //             },
    //             Beacon {
    //                 x: -364,
    //                 y: -763,
    //                 z: -893,
    //             },
    //             Beacon {
    //                 x: 807,
    //                 y: -499,
    //                 z: -711,
    //             },
    //             Beacon {
    //                 x: 755,
    //                 y: -354,
    //                 z: -619,
    //             },
    //             Beacon {
    //                 x: 553,
    //                 y: 889,
    //                 z: -390,
    //             },
    //         ],
    //     },
    //     Scanner {
    //         beacons: vec![
    //             Beacon {
    //                 x: 649,
    //                 y: 640,
    //                 z: 665,
    //             },
    //             Beacon {
    //                 x: 682,
    //                 y: -795,
    //                 z: 504,
    //             },
    //             Beacon {
    //                 x: -784,
    //                 y: 533,
    //                 z: -524,
    //             },
    //             Beacon {
    //                 x: -644,
    //                 y: 584,
    //                 z: -595,
    //             },
    //             Beacon {
    //                 x: -588,
    //                 y: -843,
    //                 z: 648,
    //             },
    //             Beacon {
    //                 x: -30,
    //                 y: 6,
    //                 z: 44,
    //             },
    //             Beacon {
    //                 x: -674,
    //                 y: 560,
    //                 z: 763,
    //             },
    //             Beacon {
    //                 x: 500,
    //                 y: 723,
    //                 z: -460,
    //             },
    //             Beacon {
    //                 x: 609,
    //                 y: 671,
    //                 z: -379,
    //             },
    //             Beacon {
    //                 x: -555,
    //                 y: -800,
    //                 z: 653,
    //             },
    //             Beacon {
    //                 x: -675,
    //                 y: -892,
    //                 z: -343,
    //             },
    //             Beacon {
    //                 x: 697,
    //                 y: -426,
    //                 z: -610,
    //             },
    //             Beacon {
    //                 x: 578,
    //                 y: 704,
    //                 z: 681,
    //             },
    //             Beacon {
    //                 x: 493,
    //                 y: 664,
    //                 z: -388,
    //             },
    //             Beacon {
    //                 x: -671,
    //                 y: -858,
    //                 z: 530,
    //             },
    //             Beacon {
    //                 x: -667,
    //                 y: 343,
    //                 z: 800,
    //             },
    //             Beacon {
    //                 x: 571,
    //                 y: -461,
    //                 z: -707,
    //             },
    //             Beacon {
    //                 x: -138,
    //                 y: -166,
    //                 z: 112,
    //             },
    //             Beacon {
    //                 x: -889,
    //                 y: 563,
    //                 z: -600,
    //             },
    //             Beacon {
    //                 x: 646,
    //                 y: -828,
    //                 z: 498,
    //             },
    //             Beacon {
    //                 x: 640,
    //                 y: 759,
    //                 z: 510,
    //             },
    //             Beacon {
    //                 x: -630,
    //                 y: 509,
    //                 z: 768,
    //             },
    //             Beacon {
    //                 x: -681,
    //                 y: -892,
    //                 z: -333,
    //             },
    //             Beacon {
    //                 x: 673,
    //                 y: -379,
    //                 z: -804,
    //             },
    //             Beacon {
    //                 x: -742,
    //                 y: -814,
    //                 z: -386,
    //             },
    //             Beacon {
    //                 x: 577,
    //                 y: -820,
    //                 z: 562,
    //             },
    //         ],
    //     },
    //     Scanner {
    //         beacons: vec![
    //             Beacon {
    //                 x: -589,
    //                 y: 542,
    //                 z: 597,
    //             },
    //             Beacon {
    //                 x: 605,
    //                 y: -692,
    //                 z: 669,
    //             },
    //             Beacon {
    //                 x: -500,
    //                 y: 565,
    //                 z: -823,
    //             },
    //             Beacon {
    //                 x: -660,
    //                 y: 373,
    //                 z: 557,
    //             },
    //             Beacon {
    //                 x: -458,
    //                 y: -679,
    //                 z: -417,
    //             },
    //             Beacon {
    //                 x: -488,
    //                 y: 449,
    //                 z: 543,
    //             },
    //             Beacon {
    //                 x: -626,
    //                 y: 468,
    //                 z: -788,
    //             },
    //             Beacon {
    //                 x: 338,
    //                 y: -750,
    //                 z: -386,
    //             },
    //             Beacon {
    //                 x: 528,
    //                 y: -832,
    //                 z: -391,
    //             },
    //             Beacon {
    //                 x: 562,
    //                 y: -778,
    //                 z: 733,
    //             },
    //             Beacon {
    //                 x: -938,
    //                 y: -730,
    //                 z: 414,
    //             },
    //             Beacon {
    //                 x: 543,
    //                 y: 643,
    //                 z: -506,
    //             },
    //             Beacon {
    //                 x: -524,
    //                 y: 371,
    //                 z: -870,
    //             },
    //             Beacon {
    //                 x: 407,
    //                 y: 773,
    //                 z: 750,
    //             },
    //             Beacon {
    //                 x: -104,
    //                 y: 29,
    //                 z: 83,
    //             },
    //             Beacon {
    //                 x: 378,
    //                 y: -903,
    //                 z: -323,
    //             },
    //             Beacon {
    //                 x: -778,
    //                 y: -728,
    //                 z: 485,
    //             },
    //             Beacon {
    //                 x: 426,
    //                 y: 699,
    //                 z: 580,
    //             },
    //             Beacon {
    //                 x: -438,
    //                 y: -605,
    //                 z: -362,
    //             },
    //             Beacon {
    //                 x: -469,
    //                 y: -447,
    //                 z: -387,
    //             },
    //             Beacon {
    //                 x: 509,
    //                 y: 732,
    //                 z: 623,
    //             },
    //             Beacon {
    //                 x: 647,
    //                 y: 635,
    //                 z: -688,
    //             },
    //             Beacon {
    //                 x: -868,
    //                 y: -804,
    //                 z: 481,
    //             },
    //             Beacon {
    //                 x: 614,
    //                 y: -800,
    //                 z: 639,
    //             },
    //             Beacon {
    //                 x: 595,
    //                 y: 780,
    //                 z: -596,
    //             },
    //         ],
    //     },
    //     Scanner {
    //         beacons: vec![
    //             Beacon {
    //                 x: 727,
    //                 y: 592,
    //                 z: 562,
    //             },
    //             Beacon {
    //                 x: -293,
    //                 y: -554,
    //                 z: 779,
    //             },
    //             Beacon {
    //                 x: 441,
    //                 y: 611,
    //                 z: -461,
    //             },
    //             Beacon {
    //                 x: -714,
    //                 y: 465,
    //                 z: -776,
    //             },
    //             Beacon {
    //                 x: -743,
    //                 y: 427,
    //                 z: -804,
    //             },
    //             Beacon {
    //                 x: -660,
    //                 y: -479,
    //                 z: -426,
    //             },
    //             Beacon {
    //                 x: 832,
    //                 y: -632,
    //                 z: 460,
    //             },
    //             Beacon {
    //                 x: 927,
    //                 y: -485,
    //                 z: -438,
    //             },
    //             Beacon {
    //                 x: 408,
    //                 y: 393,
    //                 z: -506,
    //             },
    //             Beacon {
    //                 x: 466,
    //                 y: 436,
    //                 z: -512,
    //             },
    //             Beacon {
    //                 x: 110,
    //                 y: 16,
    //                 z: 151,
    //             },
    //             Beacon {
    //                 x: -258,
    //                 y: -428,
    //                 z: 682,
    //             },
    //             Beacon {
    //                 x: -393,
    //                 y: 719,
    //                 z: 612,
    //             },
    //             Beacon {
    //                 x: -211,
    //                 y: -452,
    //                 z: 876,
    //             },
    //             Beacon {
    //                 x: 808,
    //                 y: -476,
    //                 z: -593,
    //             },
    //             Beacon {
    //                 x: -575,
    //                 y: 615,
    //                 z: 604,
    //             },
    //             Beacon {
    //                 x: -485,
    //                 y: 667,
    //                 z: 467,
    //             },
    //             Beacon {
    //                 x: -680,
    //                 y: 325,
    //                 z: -822,
    //             },
    //             Beacon {
    //                 x: -627,
    //                 y: -443,
    //                 z: -432,
    //             },
    //             Beacon {
    //                 x: 872,
    //                 y: -547,
    //                 z: -609,
    //             },
    //             Beacon {
    //                 x: 833,
    //                 y: 512,
    //                 z: 582,
    //             },
    //             Beacon {
    //                 x: 807,
    //                 y: 604,
    //                 z: 487,
    //             },
    //             Beacon {
    //                 x: 839,
    //                 y: -516,
    //                 z: 451,
    //             },
    //             Beacon {
    //                 x: 891,
    //                 y: -625,
    //                 z: 532,
    //             },
    //             Beacon {
    //                 x: -652,
    //                 y: -548,
    //                 z: -490,
    //             },
    //             Beacon {
    //                 x: 30,
    //                 y: -46,
    //                 z: -14,
    //             },
    //         ],
    //     },
    // ];

    let mut offsets = HashMap::new();
    for (left_scanner_index, left_scanner) in scanners.iter().enumerate() {
        for (right_scanner_index, right_scanner) in scanners.iter().enumerate() {
            println!(
                "left: {}, right: {}...",
                left_scanner_index, right_scanner_index
            );
            if left_scanner_index == right_scanner_index {
                continue;
            }

            if let Some(offset) =
                left_scanner.offset_with_permutations_and_flippings_to(right_scanner)
            {
                offsets.insert((left_scanner_index, right_scanner_index), offset);
            }
        }
    }
    // println!("{:#?}", offsets);

    let mut positions = HashMap::from([(0, vec![])]);
    let mut dirty = true;
    while dirty {
        dirty = false;
        for ((left_scanner_index, right_scanner_index), (permutation, flipping, offset)) in
            offsets.iter()
        {
            if positions.contains_key(left_scanner_index)
                && !positions.contains_key(right_scanner_index)
            {
                dirty = true;
                let mut rotations = positions[left_scanner_index].clone();
                rotations.push((*permutation, *flipping, *offset));
                positions.insert(*right_scanner_index, rotations);
            }
        }
    }
    // println!("{:#?}", positions);

    let final_positions = positions
        .iter()
        .map(|(scanner_index, rotations)| {
            let mut reference_beacon = Beacon { x: 0, y: 0, z: 0 };
            for (permutation, flipping, offset) in rotations.iter().rev() {
                reference_beacon.rotate(*permutation, *flipping);
                reference_beacon.x += offset.0;
                reference_beacon.y += offset.1;
                reference_beacon.z += offset.2;
            }
            (
                *scanner_index,
                (reference_beacon.x, reference_beacon.y, reference_beacon.z),
            )
        })
        .collect::<HashMap<_, _>>();

    let rotated_scanners = scanners
        .iter()
        .enumerate()
        .map(|(scanner_index, scanner)| {
            let rotations = &positions[&scanner_index];
            let mut scanner = scanner.clone();
            for (permutation, flipping, offset) in rotations.iter().rev() {
                scanner.rotate(*permutation, *flipping);
                scanner.beacons = scanner
                    .beacons
                    .iter()
                    .map(|beacon| Beacon {
                        x: beacon.x + offset.0,
                        y: beacon.y + offset.1,
                        z: beacon.z + offset.2,
                    })
                    .collect();
            }
            scanner
        })
        .collect::<Vec<_>>();

    let mut beacons = HashSet::new();
    for rotated_scanner in rotated_scanners {
        for beacon in rotated_scanner.beacons {
            beacons.insert(beacon);
        }
    }
    println!("Part One: {}", beacons.len());

    let mut largest_manhattan_distance = 0;
    for (_left_scanner_index, left_final_position) in final_positions.iter() {
        for (_right_scanner_index, right_final_position) in final_positions.iter() {
            let manhattan_distance = (left_final_position.0 - right_final_position.0).abs()
                + (left_final_position.1 - right_final_position.1).abs()
                + (left_final_position.2 - right_final_position.2).abs();
            largest_manhattan_distance = max(largest_manhattan_distance, manhattan_distance);
        }
    }
    println!("Part Two: {}", largest_manhattan_distance);
}

#[derive(Clone, Copy, Debug)]
enum Permutation {
    XYZtoXYZ,
    XYZtoXZY,
    XYZtoYXZ,
    XYZtoYZX,
    XYZtoZXY,
    XYZtoZYX,
}

#[derive(Clone, Copy, Debug)]
enum Flipping {
    NoFlips,
    ZFlipped,
    YFlipped,
    YZFlipped,
    XFlipped,
    XZFlipped,
    XYFlipped,
    XYZFlipped,
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn offset_with_permutations_and_flippings_to(
        &self,
        other: &Scanner,
    ) -> Option<(Permutation, Flipping, (isize, isize, isize))> {
        let permutations = [
            Permutation::XYZtoXYZ,
            Permutation::XYZtoXZY,
            Permutation::XYZtoYXZ,
            Permutation::XYZtoYZX,
            Permutation::XYZtoZXY,
            Permutation::XYZtoZYX,
        ];
        let flippings = [
            Flipping::NoFlips,
            Flipping::ZFlipped,
            Flipping::YFlipped,
            Flipping::YZFlipped,
            Flipping::XFlipped,
            Flipping::XZFlipped,
            Flipping::XYFlipped,
            Flipping::XYZFlipped,
        ];
        let mut result = None;
        for permutation in permutations.iter() {
            for flipping in flippings.iter() {
                let mut other = other.clone();
                other.rotate(*permutation, *flipping);
                if let Some(offset) = self.offset_to(&other) {
                    assert!(result.is_none());
                    result = Some((*permutation, *flipping, offset));
                }
            }
        }
        result
    }

    fn offset_to(&self, other: &Scanner) -> Option<(isize, isize, isize)> {
        let mut distances = HashMap::new();
        for (beacon_index_self, beacon_self) in self.beacons.iter().enumerate() {
            for (beacon_index_other, beacon_other) in other.beacons.iter().enumerate() {
                let distance = beacon_self.distance(beacon_other);
                let distances_entry = distances.entry(distance).or_insert(vec![]);
                distances_entry.push((beacon_index_self, beacon_index_other));
            }
        }
        let mut offset = None;
        for (distance, beacon_indices) in distances.iter() {
            if beacon_indices.len() >= 12 {
                offset = Some(*distance);
            }
        }
        offset
    }

    fn rotate(&mut self, permutation: Permutation, flipping: Flipping) {
        for beacon in self.beacons.iter_mut() {
            beacon.rotate(permutation, flipping);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Beacon {
    x: isize,
    y: isize,
    z: isize,
}

impl Beacon {
    fn distance(&self, other: &Beacon) -> (isize, isize, isize) {
        (self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn rotate(&mut self, permutation: Permutation, flipping: Flipping) {
        let x = self.x;
        let y = self.y;
        let z = self.z;

        match permutation {
            Permutation::XYZtoXYZ => {
                self.x = x;
                self.y = y;
                self.z = z;
            }
            Permutation::XYZtoXZY => {
                self.x = x;
                self.y = z;
                self.z = y;
            }
            Permutation::XYZtoYXZ => {
                self.x = y;
                self.y = x;
                self.z = z;
            }
            Permutation::XYZtoYZX => {
                self.x = y;
                self.y = z;
                self.z = x;
            }
            Permutation::XYZtoZXY => {
                self.x = z;
                self.y = x;
                self.z = y;
            }
            Permutation::XYZtoZYX => {
                self.x = z;
                self.y = y;
                self.z = x;
            }
        }

        match flipping {
            Flipping::NoFlips => {}
            Flipping::ZFlipped => {
                self.z = -self.z;
            }
            Flipping::YFlipped => {
                self.y = -self.y;
            }
            Flipping::YZFlipped => {
                self.y = -self.y;
                self.z = -self.z;
            }
            Flipping::XFlipped => {
                self.x = -self.x;
            }
            Flipping::XZFlipped => {
                self.x = -self.x;
                self.z = -self.z;
            }
            Flipping::XYFlipped => {
                self.x = -self.x;
                self.y = -self.y;
            }
            Flipping::XYZFlipped => {
                self.x = -self.x;
                self.y = -self.y;
                self.z = -self.z;
            }
        }
    }
}
