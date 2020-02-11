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


// C
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<VI> d(n);
  map<VI, int> occ;
  REP(i, 0, n) {
    string s;
    cin >> s;
    int m = s.size();
    VI dif(m - 1);
    REP(i, 0, m - 1) {
      int a = s[i] - 'a';
      int b = s[i + 1] - 'a';
      dif[i] = (b - a + 26) % 26;
    }
    d[i] = dif;
    occ[d[i]]++;
  }
  REP(i, 0, n) {
    cout << (occ[d[i]] >= 2 ? 1 : 0) << "\n";
  }
}
