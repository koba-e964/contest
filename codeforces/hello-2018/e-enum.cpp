#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

vector<string> ans(256);
void init(void) {
  ans[0] = "!x&x";
  ans[1] = "x&y&z";
  ans[2] = "!z&x&y";
  ans[3] = "x&y";
  ans[4] = "!y&x&z";
  ans[5] = "x&z";
  ans[6] = "!y&x&z|!z&x&y";
  ans[7] = "(y|z)&x";
  ans[8] = "!y&!z&x";
  ans[9] = "!y&!z&x|x&y&z";
  ans[10] = "!z&x";
  ans[11] = "!z&x|x&y";
  ans[12] = "!y&x";
  ans[13] = "!y&x|x&z";
  ans[14] = "!(y&z)&x";
  ans[15] = "x";
  ans[16] = "!x&y&z";
  ans[17] = "y&z";
  ans[18] = "!x&y&z|!z&x&y";
  ans[19] = "(x|z)&y";
  ans[20] = "!x&y&z|!y&x&z";
  ans[21] = "(x|y)&z";
  ans[22] = "!x&y&z|!y&x&z|!z&x&y";
  ans[23] = "(x|y)&z|x&y";
  ans[24] = "!x&y&z|!y&!z&x";
  ans[25] = "!y&!z&x|y&z";
  ans[26] = "!x&y&z|!z&x";
  ans[27] = "!z&x|y&z";
  ans[28] = "!x&y&z|!y&x";
  ans[29] = "!y&x|y&z";
  ans[30] = "!(y&z)&x|!x&y&z";
  ans[31] = "x|y&z";
  ans[32] = "!x&!z&y";
  ans[33] = "!x&!z&y|x&y&z";
  ans[34] = "!z&y";
  ans[35] = "!z&y|x&y";
  ans[36] = "!x&!z&y|!y&x&z";
  ans[37] = "!x&!z&y|x&z";
  ans[38] = "!y&x&z|!z&y";
  ans[39] = "!z&y|x&z";
  ans[40] = "!(!x&!y|x&y|z)";
  ans[41] = "!(!x&!y|x&y|z)|x&y&z";
  ans[42] = "!z&(x|y)";
  ans[43] = "!z&(x|y)|x&y";
  ans[44] = "!x&!z&y|!y&x";
  ans[45] = "!x&!z&y|!y&x|x&z";
  ans[46] = "!y&x|!z&y";
  ans[47] = "!z&y|x";
  ans[48] = "!x&y";
  ans[49] = "!x&y|y&z";
  ans[50] = "!(x&z)&y";
  ans[51] = "y";
  ans[52] = "!x&y|!y&x&z";
  ans[53] = "!x&y|x&z";
  ans[54] = "!(x&z)&y|!y&x&z";
  ans[55] = "x&z|y";
  ans[56] = "!x&y|!y&!z&x";
  ans[57] = "!x&y|!y&!z&x|y&z";
  ans[58] = "!x&y|!z&x";
  ans[59] = "!z&x|y";
  ans[60] = "!x&y|!y&x";
  ans[61] = "!x&y|!y&x|x&z";
  ans[62] = "!(x&z)&y|!y&x";
  ans[63] = "x|y";
  ans[64] = "!x&!y&z";
  ans[65] = "!x&!y&z|x&y&z";
  ans[66] = "!x&!y&z|!z&x&y";
  ans[67] = "!x&!y&z|x&y";
  ans[68] = "!y&z";
  ans[69] = "!y&z|x&z";
  ans[70] = "!y&z|!z&x&y";
  ans[71] = "!y&z|x&y";
  ans[72] = "!(!x&!z|x&z|y)";
  ans[73] = "!(!x&!z|x&z|y)|x&y&z";
  ans[74] = "!x&!y&z|!z&x";
  ans[75] = "!x&!y&z|!z&x|x&y";
  ans[76] = "!y&(x|z)";
  ans[77] = "!y&(x|z)|x&z";
  ans[78] = "!y&z|!z&x";
  ans[79] = "!y&z|x";
  ans[80] = "!x&z";
  ans[81] = "!x&z|y&z";
  ans[82] = "!x&z|!z&x&y";
  ans[83] = "!x&z|x&y";
  ans[84] = "!(x&y)&z";
  ans[85] = "z";
  ans[86] = "!(x&y)&z|!z&x&y";
  ans[87] = "x&y|z";
  ans[88] = "!x&z|!y&!z&x";
  ans[89] = "!x&z|!y&!z&x|y&z";
  ans[90] = "!x&z|!z&x";
  ans[91] = "!x&z|!z&x|x&y";
  ans[92] = "!x&z|!y&x";
  ans[93] = "!y&x|z";
  ans[94] = "!(x&y)&z|!z&x";
  ans[95] = "x|z";
  ans[96] = "!(!y&!z|x|y&z)";
  ans[97] = "!(!y&!z|x|y&z)|x&y&z";
  ans[98] = "!x&!y&z|!z&y";
  ans[99] = "!x&!y&z|!z&y|x&y";
  ans[100] = "!x&!z&y|!y&z";
  ans[101] = "!x&!z&y|!y&z|x&z";
  ans[102] = "!y&z|!z&y";
  ans[103] = "!y&z|!z&y|x&y";
  ans[104] = "!(!x&!y|x&y|z)|!x&!y&z";
  ans[105] = "!(!x&!y|x&y|z)|!x&!y&z|x&y&z";
  ans[106] = "!x&!y&z|!z&(x|y)";
  ans[107] = "!x&!y&z|!z&(x|y)|x&y";
  ans[108] = "!x&!z&y|!y&(x|z)";
  ans[109] = "!x&!z&y|!y&(x|z)|x&z";
  ans[110] = "!y&(x|z)|!z&y";
  ans[111] = "!y&z|!z&y|x";
  ans[112] = "!x&(y|z)";
  ans[113] = "!x&(y|z)|y&z";
  ans[114] = "!x&z|!z&y";
  ans[115] = "!x&z|y";
  ans[116] = "!x&y|!y&z";
  ans[117] = "!x&y|z";
  ans[118] = "!(x&y)&z|!z&y";
  ans[119] = "y|z";
  ans[120] = "!x&(y|z)|!y&!z&x";
  ans[121] = "!x&(y|z)|!y&!z&x|y&z";
  ans[122] = "!x&(y|z)|!z&x";
  ans[123] = "!x&z|!z&x|y";
  ans[124] = "!x&(y|z)|!y&x";
  ans[125] = "!x&y|!y&x|z";
  ans[126] = "!x&y|!y&z|!z&x";
  ans[127] = "x|y|z";
  ans[128] = "!(x|y|z)";
  ans[129] = "!(x|y|z)|x&y&z";
  ans[130] = "!(!x&y|!y&x|z)";
  ans[131] = "!(x|y|z)|x&y";
  ans[132] = "!(!x&z|!z&x|y)";
  ans[133] = "!(x|y|z)|x&z";
  ans[134] = "!(!x&y|!y&x|z)|!y&x&z";
  ans[135] = "!(x|y|z)|(y|z)&x";
  ans[136] = "!y&!z";
  ans[137] = "!y&!z|x&y&z";
  ans[138] = "!(!x&y|z)";
  ans[139] = "!y&!z|x&y";
  ans[140] = "!(!x&z|y)";
  ans[141] = "!y&!z|x&z";
  ans[142] = "!(!x&y|z)|!y&x";
  ans[143] = "!y&!z|x";
  ans[144] = "!(!y&z|!z&y|x)";
  ans[145] = "!(x|y|z)|y&z";
  ans[146] = "!(!x&y|!y&x|z)|!x&y&z";
  ans[147] = "!(x|y|z)|(x|z)&y";
  ans[148] = "!(!x&z|!z&x|y)|!x&y&z";
  ans[149] = "!(x|y|z)|(x|y)&z";
  ans[150] = "!(!x&y|!y&x|z)|!x&y&z|!y&x&z";
  ans[151] = "!(x|y|z)|(x|y)&z|x&y";
  ans[152] = "!x&y&z|!y&!z";
  ans[153] = "!y&!z|y&z";
  ans[154] = "!(!x&y|z)|!x&y&z";
  ans[155] = "!(!x&y|z)|y&z";
  ans[156] = "!(!x&z|y)|!x&y&z";
  ans[157] = "!(!x&z|y)|y&z";
  ans[158] = "!(!x&y|z)|!x&y&z|!y&x";
  ans[159] = "!y&!z|x|y&z";
  ans[160] = "!x&!z";
  ans[161] = "!x&!z|x&y&z";
  ans[162] = "!(!y&x|z)";
  ans[163] = "!x&!z|x&y";
  ans[164] = "!x&!z|!y&x&z";
  ans[165] = "!x&!z|x&z";
  ans[166] = "!(!y&x|z)|!y&x&z";
  ans[167] = "!(!y&x|z)|x&z";
  ans[168] = "!(x&y|z)";
  ans[169] = "!(x&y|z)|x&y&z";
  ans[170] = "!z";
  ans[171] = "!z|x&y";
  ans[172] = "!x&!z|!y&x";
  ans[173] = "!(x&y|z)|x&z";
  ans[174] = "!y&x|!z";
  ans[175] = "!z|x";
  ans[176] = "!(!y&z|x)";
  ans[177] = "!x&!z|y&z";
  ans[178] = "!(!y&x|z)|!x&y";
  ans[179] = "!x&!z|y";
  ans[180] = "!(!y&z|x)|!y&x&z";
  ans[181] = "!(!y&z|x)|x&z";
  ans[182] = "!(!y&x|z)|!x&y|!y&x&z";
  ans[183] = "!x&!z|x&z|y";
  ans[184] = "!x&y|!y&!z";
  ans[185] = "!(x&y|z)|y&z";
  ans[186] = "!x&y|!z";
  ans[187] = "!z|y";
  ans[188] = "!(!x&!y&z|x&y)";
  ans[189] = "!x&!z|!y&x|y&z";
  ans[190] = "!x&y|!y&x|!z";
  ans[191] = "!z|x|y";
  ans[192] = "!x&!y";
  ans[193] = "!x&!y|x&y&z";
  ans[194] = "!x&!y|!z&x&y";
  ans[195] = "!x&!y|x&y";
  ans[196] = "!(!z&x|y)";
  ans[197] = "!x&!y|x&z";
  ans[198] = "!(!z&x|y)|!z&x&y";
  ans[199] = "!(!z&x|y)|x&y";
  ans[200] = "!(x&z|y)";
  ans[201] = "!(x&z|y)|x&y&z";
  ans[202] = "!x&!y|!z&x";
  ans[203] = "!(x&z|y)|x&y";
  ans[204] = "!y";
  ans[205] = "!y|x&z";
  ans[206] = "!y|!z&x";
  ans[207] = "!y|x";
  ans[208] = "!(!z&y|x)";
  ans[209] = "!x&!y|y&z";
  ans[210] = "!(!z&y|x)|!z&x&y";
  ans[211] = "!(!z&y|x)|x&y";
  ans[212] = "!(!z&x|y)|!x&z";
  ans[213] = "!x&!y|z";
  ans[214] = "!(!z&x|y)|!x&z|!z&x&y";
  ans[215] = "!x&!y|x&y|z";
  ans[216] = "!x&z|!y&!z";
  ans[217] = "!(x&z|y)|y&z";
  ans[218] = "!(!x&!z&y|x&z)";
  ans[219] = "!x&!y|!z&x|y&z";
  ans[220] = "!x&z|!y";
  ans[221] = "!y|z";
  ans[222] = "!x&z|!y|!z&x";
  ans[223] = "!y|x|z";
  ans[224] = "!(x|y&z)";
  ans[225] = "!(x|y&z)|x&y&z";
  ans[226] = "!x&!y|!z&y";
  ans[227] = "!(x|y&z)|x&y";
  ans[228] = "!x&!z|!y&z";
  ans[229] = "!(x|y&z)|x&z";
  ans[230] = "!(!y&!z&x|y&z)";
  ans[231] = "!x&!y|!z&y|x&z";
  ans[232] = "!((x|y)&z|x&y)";
  ans[233] = "!((x|y)&z|x&y)|x&y&z";
  ans[234] = "!x&!y|!z";
  ans[235] = "!x&!y|!z|x&y";
  ans[236] = "!x&!z|!y";
  ans[237] = "!x&!z|!y|x&z";
  ans[238] = "!y|!z";
  ans[239] = "!y|!z|x";
  ans[240] = "!x";
  ans[241] = "!x|y&z";
  ans[242] = "!x|!z&y";
  ans[243] = "!x|y";
  ans[244] = "!x|!y&z";
  ans[245] = "!x|z";
  ans[246] = "!x|!y&z|!z&y";
  ans[247] = "!x|y|z";
  ans[248] = "!x|!y&!z";
  ans[249] = "!x|!y&!z|y&z";
  ans[250] = "!x|!z";
  ans[251] = "!x|!z|y";
  ans[252] = "!x|!y";
  ans[253] = "!x|!y|z";
  ans[254] = "!(x&y&z)";
  ans[255] = "!x|x";
}

int main(void) {
  init();
  REP(bits, 0, 256) {
    cout << ans[bits] << endl;
  }
}
