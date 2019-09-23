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
  int d, sh, th, sm, tm;
  cin >> d >> sh >> sm >> th >> tm;
  int lo, hi;
  if (d >= 6) {
    lo = 7 * 60;
    hi = 22 * 60;
  } else {
    lo = 19 * 60;
    hi = 22 * 60;
  }
  int s = sh * 60 + sm;
  int t = th * 60 + tm;
  cout << (s >= lo && t <= hi ? "Yay!" : ":(") << endl;
}
