#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

char s[1000100];
char t[1000100];

int main(void) {
  scanf("%s", s);
  scanf("%s", t);
  int sl = strlen(s);
  int tl = strlen(t);
  VL sf(26), tf(26);
  ll que = 0;
  REP(i, 0, sl) {
    if (s[i] == '?') que += 1;
    else sf[s[i] - 'a'] += 1;
  }
  REP(i, 0, tl) {
    tf[t[i] - 'a'] += 1;
  }
  ll pass = 0, fail = sl / tl + 1;
  while (fail - pass > 1) {
    ll mid = (fail + pass) / 2;
    ll sh = 0;
    REP(i, 0, 26) {
      if (sf[i] < mid * tf[i]) {
	sh += mid * tf[i] - sf[i];
      }
    }
    if (sh <= que) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  VL rem(26);
  REP(i, 0, 26) {
    rem[i] = max(0LL, pass * tf[i] - sf[i]);
  }
  int pos = 0;
  REP(i, 0, sl) {
    if (s[i] != '?') { continue; }
    while (pos < 26 && rem[pos] <= 0) pos++;
    s[i] = pos < 26 ? 'a' + pos : 'x';
    if (pos < 26) {
      rem[pos]--;
    }
  }
  puts(s);
}
