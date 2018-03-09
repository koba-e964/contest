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

const int N = 210000;
VI occ[N];

void fail(void) {
  cout << "-1\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.length();
  int hi = 0;
  int ma = 0;
  REP(i, 0, n) {
    if (s[i] == '0') {
      occ[hi].push_back(i);
      hi++;
    } else {
      if (hi <= 0) {
	fail();
      }
      hi--;
      occ[hi].push_back(i);
    }
    ma = max(ma, hi);
  }
  REP(i, 0, ma) {
    if (occ[i].size() % 2 == 0) {
      fail();
    }
  }
  cout << ma << "\n";
  REP(i, 0, ma) {
    cout << occ[i].size();
    REP(j, 0, occ[i].size()) {
      cout << " " << occ[i][j] + 1;
    }
    cout << "\n";
  }
}
