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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<string> s(n);
  VI x(n);
  REP(i, 0, n) cin >> s[i] >> x[i];
  int zero = 0, one = 1023;
  REP(i, 0, n) {
    if (s[i] == "|") {
      zero |= x[i];
      one |= x[i];
    }
    if (s[i] == "&") {
      zero &= x[i];
      one &= x[i];
    }
    if (s[i] == "^") {
      zero ^= x[i];
      one ^= x[i];
    }
  }
  int p0 = 0, p1 = 0, p2 = 0;
  REP(i, 0, 10) {
    int z = zero >> i & 1;
    int o = one >> i & 1;
    if (z == 0 && o == 0) {
    }
    if (z == 0 && o == 1) {
      p2 |= 1 << i;
    }
    if (z == 1 && o == 0) {
      p0 |= 1 << i;
      p2 |= 1 << i;
    }
    if (z == 1 && o == 1) {
      p1 |= 1 << i;
      p2 |= 1 << i;
    }
  }
  cout << 3 << "\n";
  cout << "^ " << p0 << "\n";
  cout << "| " << p1 << "\n";
  cout << "& " << p2 << "\n";
}
