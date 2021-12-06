fn main() {
    let lines = [
        Line::new(989, 854, 521, 854),
        Line::new(831, 695, 402, 266),
        Line::new(38, 805, 306, 537),
        Line::new(802, 24, 802, 824),
        Line::new(951, 478, 951, 758),
        Line::new(508, 987, 508, 868),
        Line::new(602, 246, 108, 246),
        Line::new(765, 781, 76, 92),
        Line::new(248, 757, 644, 361),
        Line::new(296, 987, 296, 958),
        Line::new(240, 430, 240, 36),
        Line::new(672, 530, 156, 14),
        Line::new(920, 19, 425, 514),
        Line::new(737, 389, 859, 389),
        Line::new(762, 364, 468, 364),
        Line::new(889, 114, 45, 958),
        Line::new(961, 749, 843, 749),
        Line::new(528, 905, 528, 50),
        Line::new(515, 691, 768, 438),
        Line::new(718, 970, 718, 66),
        Line::new(335, 149, 105, 149),
        Line::new(543, 879, 149, 879),
        Line::new(739, 521, 340, 521),
        Line::new(322, 493, 676, 139),
        Line::new(214, 242, 214, 297),
        Line::new(455, 604, 502, 604),
        Line::new(534, 760, 755, 760),
        Line::new(90, 884, 670, 304),
        Line::new(908, 62, 689, 62),
        Line::new(684, 224, 83, 825),
        Line::new(235, 91, 46, 91),
        Line::new(686, 28, 486, 228),
        Line::new(655, 793, 282, 420),
        Line::new(36, 164, 853, 981),
        Line::new(592, 229, 246, 229),
        Line::new(454, 682, 822, 314),
        Line::new(734, 84, 718, 100),
        Line::new(970, 704, 520, 704),
        Line::new(761, 314, 761, 31),
        Line::new(420, 228, 950, 228),
        Line::new(868, 950, 209, 950),
        Line::new(476, 38, 553, 38),
        Line::new(606, 230, 404, 230),
        Line::new(804, 71, 804, 678),
        Line::new(442, 938, 442, 380),
        Line::new(962, 636, 595, 636),
        Line::new(645, 978, 73, 406),
        Line::new(737, 915, 737, 235),
        Line::new(923, 110, 923, 541),
        Line::new(578, 649, 701, 649),
        Line::new(360, 46, 403, 46),
        Line::new(442, 539, 909, 539),
        Line::new(892, 486, 291, 486),
        Line::new(956, 391, 828, 391),
        Line::new(475, 567, 459, 583),
        Line::new(22, 853, 160, 853),
        Line::new(596, 896, 241, 896),
        Line::new(961, 225, 262, 924),
        Line::new(860, 877, 201, 877),
        Line::new(767, 863, 328, 424),
        Line::new(24, 103, 418, 497),
        Line::new(967, 989, 55, 77),
        Line::new(969, 463, 90, 463),
        Line::new(184, 921, 970, 135),
        Line::new(123, 787, 897, 13),
        Line::new(44, 849, 840, 53),
        Line::new(900, 686, 389, 175),
        Line::new(865, 588, 660, 383),
        Line::new(730, 254, 730, 54),
        Line::new(654, 114, 92, 676),
        Line::new(340, 598, 540, 798),
        Line::new(114, 774, 114, 961),
        Line::new(69, 138, 909, 138),
        Line::new(727, 701, 727, 963),
        Line::new(278, 212, 439, 51),
        Line::new(384, 297, 428, 297),
        Line::new(538, 473, 538, 696),
        Line::new(848, 353, 592, 97),
        Line::new(227, 388, 355, 388),
        Line::new(122, 963, 447, 638),
        Line::new(934, 941, 167, 174),
        Line::new(317, 508, 957, 508),
        Line::new(245, 15, 977, 747),
        Line::new(590, 334, 684, 334),
        Line::new(934, 168, 321, 781),
        Line::new(305, 414, 234, 414),
        Line::new(109, 288, 142, 288),
        Line::new(135, 215, 780, 860),
        Line::new(29, 882, 358, 882),
        Line::new(88, 263, 725, 900),
        Line::new(58, 623, 58, 596),
        Line::new(772, 386, 236, 922),
        Line::new(515, 33, 515, 279),
        Line::new(16, 831, 16, 750),
        Line::new(177, 25, 221, 25),
        Line::new(32, 16, 988, 972),
        Line::new(374, 586, 162, 798),
        Line::new(99, 252, 625, 252),
        Line::new(709, 793, 740, 793),
        Line::new(31, 941, 901, 71),
        Line::new(784, 283, 111, 956),
        Line::new(223, 955, 223, 349),
        Line::new(893, 618, 893, 230),
        Line::new(378, 422, 378, 481),
        Line::new(208, 922, 208, 64),
        Line::new(966, 984, 25, 43),
        Line::new(438, 227, 82, 583),
        Line::new(807, 449, 464, 106),
        Line::new(758, 744, 219, 744),
        Line::new(331, 450, 331, 347),
        Line::new(38, 959, 986, 11),
        Line::new(819, 565, 819, 117),
        Line::new(486, 552, 392, 552),
        Line::new(792, 501, 862, 501),
        Line::new(876, 259, 101, 259),
        Line::new(423, 731, 423, 313),
        Line::new(864, 61, 446, 479),
        Line::new(794, 182, 96, 880),
        Line::new(102, 589, 736, 589),
        Line::new(811, 472, 931, 592),
        Line::new(946, 525, 946, 809),
        Line::new(358, 709, 255, 606),
        Line::new(493, 42, 687, 42),
        Line::new(865, 545, 865, 606),
        Line::new(170, 717, 260, 717),
        Line::new(16, 10, 987, 981),
        Line::new(986, 236, 986, 262),
        Line::new(922, 721, 283, 82),
        Line::new(307, 731, 759, 279),
        Line::new(935, 887, 915, 887),
        Line::new(591, 178, 481, 178),
        Line::new(245, 211, 734, 700),
        Line::new(172, 290, 172, 838),
        Line::new(63, 20, 975, 932),
        Line::new(450, 969, 450, 89),
        Line::new(823, 878, 823, 338),
        Line::new(354, 377, 677, 54),
        Line::new(103, 939, 703, 339),
        Line::new(518, 961, 564, 915),
        Line::new(593, 353, 373, 353),
        Line::new(777, 787, 777, 527),
        Line::new(440, 337, 276, 501),
        Line::new(80, 82, 368, 370),
        Line::new(963, 48, 963, 747),
        Line::new(210, 552, 242, 552),
        Line::new(563, 31, 39, 31),
        Line::new(87, 929, 959, 57),
        Line::new(164, 383, 113, 434),
        Line::new(184, 657, 184, 932),
        Line::new(904, 118, 225, 797),
        Line::new(934, 136, 934, 925),
        Line::new(523, 873, 271, 873),
        Line::new(125, 922, 28, 922),
        Line::new(753, 744, 522, 744),
        Line::new(221, 292, 571, 292),
        Line::new(322, 929, 452, 929),
        Line::new(988, 579, 772, 579),
        Line::new(15, 451, 48, 451),
        Line::new(559, 540, 151, 948),
        Line::new(183, 609, 183, 754),
        Line::new(251, 223, 872, 844),
        Line::new(224, 344, 162, 406),
        Line::new(124, 86, 976, 938),
        Line::new(446, 690, 235, 690),
        Line::new(882, 632, 698, 816),
        Line::new(419, 380, 613, 574),
        Line::new(787, 32, 295, 32),
        Line::new(127, 245, 249, 123),
        Line::new(364, 298, 927, 298),
        Line::new(786, 325, 481, 325),
        Line::new(903, 87, 57, 933),
        Line::new(408, 326, 666, 584),
        Line::new(266, 506, 636, 876),
        Line::new(701, 295, 701, 82),
        Line::new(383, 710, 566, 710),
        Line::new(245, 984, 846, 383),
        Line::new(542, 394, 542, 827),
        Line::new(359, 220, 359, 309),
        Line::new(593, 180, 369, 180),
        Line::new(70, 27, 973, 930),
        Line::new(26, 503, 773, 503),
        Line::new(338, 371, 343, 366),
        Line::new(382, 481, 355, 481),
        Line::new(13, 43, 498, 528),
        Line::new(943, 264, 486, 264),
        Line::new(101, 504, 410, 813),
        Line::new(963, 40, 122, 881),
        Line::new(291, 224, 376, 139),
        Line::new(193, 12, 349, 168),
        Line::new(874, 214, 287, 801),
        Line::new(119, 463, 554, 898),
        Line::new(736, 343, 249, 830),
        Line::new(824, 561, 686, 561),
        Line::new(375, 589, 652, 589),
        Line::new(816, 757, 139, 80),
        Line::new(132, 759, 142, 769),
        Line::new(328, 235, 328, 183),
        Line::new(441, 800, 112, 800),
        Line::new(133, 458, 43, 368),
        Line::new(466, 474, 779, 474),
        Line::new(834, 481, 834, 441),
        Line::new(62, 340, 62, 526),
        Line::new(675, 148, 960, 433),
        Line::new(791, 924, 957, 758),
        Line::new(91, 903, 739, 903),
        Line::new(837, 33, 101, 769),
        Line::new(479, 588, 302, 588),
        Line::new(389, 362, 389, 773),
        Line::new(299, 595, 262, 632),
        Line::new(69, 473, 853, 473),
        Line::new(496, 428, 847, 428),
        Line::new(23, 199, 773, 949),
        Line::new(441, 426, 684, 669),
        Line::new(594, 132, 830, 132),
        Line::new(708, 148, 768, 148),
        Line::new(882, 336, 882, 941),
        Line::new(27, 878, 672, 233),
        Line::new(754, 827, 425, 827),
        Line::new(484, 18, 949, 18),
        Line::new(219, 92, 797, 670),
        Line::new(977, 419, 522, 874),
        Line::new(647, 679, 647, 756),
        Line::new(250, 516, 250, 707),
        Line::new(716, 808, 473, 808),
        Line::new(570, 791, 915, 791),
        Line::new(555, 723, 923, 355),
        Line::new(476, 861, 367, 861),
        Line::new(603, 531, 192, 531),
        Line::new(539, 848, 539, 717),
        Line::new(217, 395, 217, 968),
        Line::new(982, 842, 982, 383),
        Line::new(790, 363, 548, 121),
        Line::new(855, 521, 613, 763),
        Line::new(942, 30, 121, 851),
        Line::new(175, 754, 10, 754),
        Line::new(47, 959, 836, 170),
        Line::new(342, 79, 171, 79),
        Line::new(667, 110, 707, 150),
        Line::new(49, 51, 954, 956),
        Line::new(734, 547, 441, 840),
        Line::new(328, 337, 844, 853),
        Line::new(108, 572, 403, 572),
        Line::new(660, 698, 202, 240),
        Line::new(908, 690, 56, 690),
        Line::new(945, 205, 850, 300),
        Line::new(138, 462, 138, 750),
        Line::new(922, 95, 36, 981),
        Line::new(513, 136, 513, 69),
        Line::new(446, 861, 408, 861),
        Line::new(558, 845, 778, 845),
        Line::new(206, 473, 206, 587),
        Line::new(354, 344, 354, 746),
        Line::new(673, 81, 219, 81),
        Line::new(618, 734, 135, 734),
        Line::new(444, 601, 382, 539),
        Line::new(973, 764, 973, 961),
        Line::new(512, 336, 512, 826),
        Line::new(55, 305, 427, 677),
        Line::new(972, 989, 176, 193),
        Line::new(953, 309, 953, 924),
        Line::new(935, 548, 935, 693),
        Line::new(962, 131, 962, 721),
        Line::new(927, 775, 927, 706),
        Line::new(441, 561, 135, 561),
        Line::new(962, 46, 84, 924),
        Line::new(67, 837, 888, 16),
        Line::new(38, 36, 966, 964),
        Line::new(20, 114, 988, 114),
        Line::new(775, 932, 72, 229),
        Line::new(25, 376, 232, 169),
        Line::new(553, 481, 553, 130),
        Line::new(231, 495, 231, 979),
        Line::new(927, 199, 927, 404),
        Line::new(931, 667, 87, 667),
        Line::new(449, 111, 69, 111),
        Line::new(812, 692, 812, 16),
        Line::new(983, 47, 43, 987),
        Line::new(819, 104, 819, 721),
        Line::new(970, 25, 267, 728),
        Line::new(761, 147, 149, 147),
        Line::new(855, 845, 225, 215),
        Line::new(53, 541, 175, 541),
        Line::new(101, 719, 101, 567),
        Line::new(986, 980, 45, 39),
        Line::new(821, 472, 264, 472),
        Line::new(517, 396, 851, 730),
        Line::new(78, 638, 78, 816),
        Line::new(772, 989, 772, 394),
        Line::new(434, 238, 610, 414),
        Line::new(547, 493, 525, 493),
        Line::new(903, 162, 900, 162),
        Line::new(856, 105, 291, 105),
        Line::new(370, 659, 740, 659),
        Line::new(841, 194, 985, 50),
        Line::new(764, 194, 374, 584),
        Line::new(49, 428, 613, 428),
        Line::new(286, 441, 131, 441),
        Line::new(693, 928, 332, 928),
        Line::new(624, 686, 507, 803),
        Line::new(809, 31, 901, 31),
        Line::new(303, 696, 726, 696),
        Line::new(398, 157, 269, 157),
        Line::new(80, 15, 676, 611),
        Line::new(81, 199, 618, 736),
        Line::new(805, 247, 219, 833),
        Line::new(427, 208, 300, 208),
        Line::new(545, 69, 638, 162),
        Line::new(587, 518, 397, 708),
        Line::new(782, 649, 782, 894),
        Line::new(802, 793, 803, 794),
        Line::new(46, 939, 893, 92),
        Line::new(534, 512, 534, 644),
        Line::new(505, 942, 85, 522),
        Line::new(243, 479, 243, 916),
        Line::new(616, 737, 616, 462),
        Line::new(236, 31, 236, 387),
        Line::new(413, 397, 500, 397),
        Line::new(45, 833, 45, 982),
        Line::new(156, 824, 265, 715),
        Line::new(763, 920, 763, 734),
        Line::new(524, 47, 790, 313),
        Line::new(220, 859, 429, 650),
        Line::new(503, 467, 503, 546),
        Line::new(522, 454, 522, 855),
        Line::new(616, 901, 616, 706),
        Line::new(435, 593, 378, 593),
        Line::new(31, 780, 31, 868),
        Line::new(353, 844, 599, 844),
        Line::new(417, 868, 958, 327),
        Line::new(613, 949, 613, 318),
        Line::new(264, 522, 724, 522),
        Line::new(942, 924, 269, 924),
        Line::new(877, 311, 877, 102),
        Line::new(961, 55, 103, 913),
        Line::new(776, 270, 871, 270),
        Line::new(987, 28, 955, 28),
        Line::new(72, 743, 72, 434),
        Line::new(727, 435, 727, 876),
        Line::new(956, 19, 956, 417),
        Line::new(424, 124, 915, 124),
        Line::new(222, 656, 42, 836),
        Line::new(294, 137, 717, 137),
        Line::new(91, 260, 91, 165),
        Line::new(15, 979, 973, 21),
        Line::new(305, 348, 491, 348),
        Line::new(212, 408, 554, 66),
        Line::new(578, 471, 578, 925),
        Line::new(187, 85, 187, 374),
        Line::new(722, 484, 722, 837),
        Line::new(714, 926, 61, 926),
        Line::new(682, 141, 682, 268),
        Line::new(93, 502, 433, 162),
        Line::new(580, 666, 216, 666),
        Line::new(722, 612, 722, 290),
        Line::new(292, 798, 292, 504),
        Line::new(41, 973, 961, 53),
        Line::new(760, 611, 760, 883),
        Line::new(398, 221, 398, 475),
        Line::new(98, 287, 98, 165),
        Line::new(555, 754, 285, 754),
        Line::new(44, 871, 700, 215),
        Line::new(547, 56, 547, 493),
        Line::new(927, 583, 448, 104),
        Line::new(774, 383, 215, 942),
        Line::new(948, 391, 920, 391),
        Line::new(433, 528, 433, 708),
        Line::new(374, 244, 903, 244),
        Line::new(141, 491, 141, 837),
        Line::new(522, 946, 648, 946),
        Line::new(51, 712, 51, 747),
        Line::new(341, 621, 127, 621),
        Line::new(395, 364, 281, 478),
        Line::new(89, 804, 89, 91),
        Line::new(818, 157, 938, 157),
        Line::new(794, 482, 521, 482),
        Line::new(877, 402, 877, 262),
        Line::new(335, 655, 633, 655),
        Line::new(316, 333, 120, 333),
        Line::new(566, 258, 566, 131),
        Line::new(288, 102, 288, 448),
        Line::new(183, 969, 183, 849),
        Line::new(941, 970, 26, 55),
        Line::new(681, 588, 417, 324),
        Line::new(583, 537, 854, 537),
        Line::new(787, 183, 292, 678),
        Line::new(737, 30, 28, 30),
        Line::new(31, 21, 981, 971),
        Line::new(960, 980, 10, 30),
        Line::new(488, 694, 147, 694),
        Line::new(182, 905, 676, 411),
        Line::new(229, 496, 57, 496),
        Line::new(736, 794, 736, 709),
        Line::new(357, 966, 234, 843),
        Line::new(389, 525, 667, 525),
        Line::new(305, 953, 305, 870),
        Line::new(716, 649, 852, 785),
        Line::new(768, 928, 93, 253),
        Line::new(91, 173, 91, 527),
        Line::new(866, 52, 866, 367),
        Line::new(583, 469, 813, 699),
        Line::new(821, 55, 70, 806),
        Line::new(23, 218, 755, 950),
        Line::new(78, 132, 78, 41),
        Line::new(463, 976, 301, 814),
        Line::new(722, 931, 800, 931),
        Line::new(187, 820, 187, 289),
        Line::new(59, 846, 59, 423),
        Line::new(884, 108, 155, 108),
        Line::new(756, 714, 411, 714),
        Line::new(68, 926, 964, 30),
        Line::new(672, 510, 937, 510),
        Line::new(258, 581, 497, 581),
        Line::new(77, 703, 77, 626),
        Line::new(10, 12, 963, 965),
        Line::new(941, 99, 311, 99),
        Line::new(74, 27, 972, 925),
        Line::new(504, 82, 204, 382),
        Line::new(671, 402, 366, 402),
        Line::new(35, 828, 483, 380),
        Line::new(298, 464, 816, 982),
        Line::new(230, 279, 230, 458),
        Line::new(936, 325, 936, 407),
        Line::new(711, 219, 617, 219),
        Line::new(394, 852, 394, 951),
        Line::new(813, 381, 515, 381),
        Line::new(11, 37, 11, 833),
        Line::new(576, 338, 318, 596),
        Line::new(899, 179, 403, 675),
        Line::new(621, 975, 344, 698),
        Line::new(786, 299, 138, 299),
        Line::new(542, 601, 542, 932),
        Line::new(930, 820, 532, 820),
        Line::new(949, 441, 578, 441),
        Line::new(736, 88, 667, 88),
        Line::new(51, 181, 550, 181),
        Line::new(154, 599, 410, 599),
        Line::new(720, 345, 867, 345),
        Line::new(410, 321, 410, 821),
        Line::new(765, 381, 765, 152),
        Line::new(53, 247, 545, 739),
        Line::new(507, 155, 823, 471),
        Line::new(42, 939, 42, 614),
        Line::new(88, 105, 693, 105),
        Line::new(743, 188, 830, 275),
        Line::new(956, 13, 12, 957),
        Line::new(829, 35, 71, 793),
        Line::new(717, 352, 601, 468),
        Line::new(439, 44, 439, 548),
        Line::new(136, 14, 310, 188),
        Line::new(429, 119, 391, 157),
        Line::new(985, 238, 391, 832),
        Line::new(231, 814, 886, 814),
        Line::new(216, 972, 216, 584),
        Line::new(422, 742, 422, 343),
        Line::new(835, 285, 315, 805),
        Line::new(373, 146, 373, 819),
        Line::new(629, 115, 417, 115),
        Line::new(234, 296, 756, 818),
        Line::new(710, 445, 173, 982),
        Line::new(22, 109, 794, 109),
        Line::new(26, 506, 26, 56),
        Line::new(395, 685, 395, 276),
        Line::new(556, 626, 556, 345),
        Line::new(588, 747, 295, 747),
        Line::new(188, 93, 841, 93),
        Line::new(663, 313, 663, 724),
        Line::new(960, 692, 831, 563),
        Line::new(268, 511, 268, 932),
        Line::new(328, 311, 211, 311),
        Line::new(877, 74, 68, 883),
        Line::new(343, 666, 343, 341),
        Line::new(578, 517, 130, 517),
        Line::new(145, 761, 145, 412),
        Line::new(723, 269, 181, 269),
        Line::new(318, 471, 699, 471),
        Line::new(760, 768, 122, 768),
        Line::new(327, 817, 825, 319),
        Line::new(482, 253, 97, 253),
        Line::new(178, 181, 424, 427),
        Line::new(247, 429, 27, 429),
        Line::new(273, 840, 521, 840),
        Line::new(684, 819, 693, 819),
        Line::new(740, 35, 740, 953),
        Line::new(977, 25, 64, 938),
        Line::new(848, 968, 848, 114),
        Line::new(851, 34, 186, 699),
        Line::new(595, 937, 892, 640),
        Line::new(983, 654, 983, 448),
        Line::new(255, 359, 255, 58),
        Line::new(325, 507, 271, 507),
        Line::new(442, 230, 846, 230),
        Line::new(839, 895, 146, 202),
        Line::new(588, 47, 282, 353),
        Line::new(130, 485, 130, 892),
        Line::new(308, 886, 783, 886),
        Line::new(949, 681, 350, 681),
        Line::new(256, 561, 746, 561),
        Line::new(242, 119, 608, 119),
        Line::new(916, 883, 410, 377),
        Line::new(562, 433, 241, 754),
    ];

    let size = 1000;
    let mut points = vec![];
    for _ in 0..size {
        points.push(vec![0; size]);
    }

    lines_to_points(&lines, &mut points, false);

    let mut number_of_at_least_two = 0;
    for row in points {
        for amount in row {
            if amount >= 2 {
                number_of_at_least_two += 1;
            }
        }
    }
    println!("Part One: {}", number_of_at_least_two);

    let mut points = vec![];
    for _ in 0..size {
        points.push(vec![0; size]);
    }

    lines_to_points(&lines, &mut points, true);

    let mut number_of_at_least_two = 0;
    for row in points {
        for amount in row {
            if amount >= 2 {
                number_of_at_least_two += 1;
            }
        }
    }
    println!("Part Two: {}", number_of_at_least_two);
}

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

fn lines_to_points(lines: &[Line], points: &mut Vec<Vec<i32>>, with_diagonal: bool) {
    for line in lines {
        let differs_horizontal = line.x1 != line.x2;
        let differs_vertical = line.y1 != line.y2;
        let (x1, x2) = if line.x1 <= line.x2 {
            (line.x1, line.x2)
        } else {
            (line.x2, line.x1)
        };
        let (y1, y2) = if line.y1 <= line.y2 {
            (line.y1, line.y2)
        } else {
            (line.y2, line.y1)
        };
        if differs_horizontal && differs_vertical {
            if with_diagonal {
                let length = x2 - x1;
                assert_eq!(length, y2 - y1);
                let x_direction_factor = if line.x1 <= line.x2 { 1 } else { -1 };
                let y_direction_factor = if line.y1 <= line.y2 { 1 } else { -1 };
                for i in 0..=length {
                    points[(line.y1 + (i * y_direction_factor)) as usize]
                        [(line.x1 + (i * x_direction_factor)) as usize] += 1;
                }
            }
        } else if differs_horizontal {
            let y = line.y1;
            for x in x1..=x2 {
                points[y as usize][x as usize] += 1;
            }
        } else {
            let x = line.x1;
            for y in y1..=y2 {
                points[y as usize][x as usize] += 1;
            }
        }
    }
}