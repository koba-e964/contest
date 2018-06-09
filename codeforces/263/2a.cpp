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
  int dx[4] = {1, 0, -1, 0}, dy[4] = {0, 1, 0, -1};
  bool ok = true;
  REP(i, 0, n) {
    REP(j, 0, n) {
      int cnt = 0;
      REP(d, 0, 4) {
        int nx = i + dx[d], ny = j + dy[d];
        if (nx < 0 || nx >= n || ny < 0 || ny >= n) continue;
        cnt += s[nx][ny] == 'o';
      }
      if (cnt % 2 != 0) ok = false;
    }
  }
  cout << (ok ? "YES" : "NO") << "\n";
}
