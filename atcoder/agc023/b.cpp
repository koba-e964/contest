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
  REP(i, 0, n) cin >> s[i];
  int tot = 0;
  REP(diff, 0, n) {
    bool ok = true;
    REP(i, 0, n) {
      REP(j, 0, n) {
	int ni = (diff + i) % n;
	int nj = (j - diff + n) % n;
	if (s[i][j] != s[nj][ni]) ok = false;
      }
    }
    tot += ok ? 1 : 0;
  }
  cout << tot * n << endl;
}
