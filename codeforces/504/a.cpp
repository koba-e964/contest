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
  int n, m;
  string s, t;
  cin >> n >> m >> s >> t;
  bool ok = false;
  if (s.find('*') != string::npos) {
    int k = s.find('*');
    ok = s.size() <= t.size() + 1 &&
      s.substr(0, k) == t.substr(0, k) && s.substr(k + 1) == t.substr(t.size() - s.size() + k + 1);
  } else {
    ok = s == t;
  }
  cout << (ok ? "YES" : "NO") << "\n";
}
