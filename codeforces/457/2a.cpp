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
  int x, hh, mm;
  cin >> x >> hh >> mm;
  REP(i, 0, 100000) {
    int y = 60 * hh + mm + (1440 - x) * i;
    y %= 1440;
    int nh = y / 60;
    int nm = y % 60;
    if (nh % 10 == 7 || nm % 10 == 7) {
      cout << i << "\n";
      return 0;
    }
  }
}
