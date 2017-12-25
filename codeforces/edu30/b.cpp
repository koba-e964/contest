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
  string s;
  cin >> n >> s;
  VI a(n + 1, 0);
  map<int, int> mi_occ;
  REP(i, 0, n) {
    a[i + 1] = a[i] + (s[i] == '1' ? 1 : -1);
  }
  int ma = 0;
  REP(i, 0, n + 1) {
    if (mi_occ.count(a[i]) == 0) {
      mi_occ[a[i]] = i;
    } else {
      ma = max(ma, i - mi_occ[a[i]]);
    }
  }
  cout << ma << "\n";
}
