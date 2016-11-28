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

const int K = 20;


int main(void){
  int k;
  string s;
  cin >> k >> s;
  if (k == 0) {
    cout << s << endl;
    return 0;
  }
  int n = s.length();
  int q = n / (k + 1);
  int r = n % (k + 1);
  assert (n <= 16);
  string mi = "zzz";
  REP(bits, 0, 1 << (k + 1)) {
    if (__builtin_popcount(bits) != r) {
      continue;
    }
    VI tos(k + 2);
    tos[0] = 0;
    REP(i, 1, k + 2) {
      tos[i] = tos[i - 1] + q + (bits & (1 << (i - 1)) ? 1 : 0);
    }
    assert (tos[k + 1] == n);
    string ma = "";
    REP(i, 0, k + 1) {
      ma = max(ma, s.substr(tos[i], tos[i + 1] - tos[i]));
    }
    mi = min(mi, ma);
  }
  cout << mi << endl;
}
