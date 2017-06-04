#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll to_pat(const string &v) {
  ll ret = 0;
  REP(i, 0, v.length()) {
    if (v[v.length() - i - 1] % 2) {
      ret |= 1LL << i;
    }
  }
  return ret;
}

int main(void){
  int t;
  cin >> t;
  map<ll, int> st;
  REP(loop_cnt, 0, t) {
    string ty;
    string v;
    cin >> ty >> v;
    ll pat = to_pat(v);
    switch(ty[0]) {
    case '+':
      if (st.count(pat)) {
	st[pat]++;
      } else {
	st[pat] = 1;
      }
      break;
    case '-':
      assert (st.count(pat));
      st[pat]--;
      break;
    case '?':
      cout << st[pat] << endl;
      break;
    default:
      assert (0);
    }
  }
}
