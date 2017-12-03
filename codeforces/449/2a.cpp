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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  string s;
  cin >> n >> m >> s;
  REP(puella_magica, 0, m) {
    int l, r;
    char c1, c2;
    cin >> l >> r >> c1 >> c2;
    REP(i, l - 1, r) {
      if (s[i] == c1) {
	s[i] = c2;
      }
    }
  }
  cout << s << "\n";
}
