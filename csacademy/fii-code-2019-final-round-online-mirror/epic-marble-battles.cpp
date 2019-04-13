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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

bool win(const string &s) {
  VI a;
  int cur = 0;
  REP(i, 0, s.length()) {
    if (s[i] == 'b') {
      a.push_back(cur);
      cur = 0;
    } else {
      cur += 1;
    }
  }
  a.push_back(cur);
  if (DEBUG) {
    REP(i, 0, a.size()) cerr << " " << a[i];
    cerr << endl;
  }
  int fcons = 0;
  int scons = 0;
  int zcont = 0;
  REP(i, 0, a.size()) {
    scons += a[i] / 3;
    if (a[i] % 3 == 1) {
    }
    if (a[i] % 3 == 0) {
      zcont += 1;
      if (zcont >= 2) {
        fcons += 1;
        zcont -= 2;
      }
    }
    if (a[i] % 3 == 2) {
      zcont = 0; 
    }
  }
  if (DEBUG) {
    DEBUGP(fcons);
    DEBUGP(scons);
  }
  return fcons > scons;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(_, 0, t) {
    string s;
    cin >> s;
    cout << (win(s) ? "First" : "Second") << "\n";
  }
}
