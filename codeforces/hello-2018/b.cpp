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

const int N = 1010;
VI ch[N];
int p[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    cin >> p[i + 1];
    p[i + 1]--;
    ch[p[i + 1]].push_back(i + 1);
  }
  REP(i, 0, n) {
    int l = ch[i].size();
    if (l == 0) { continue; } // leaf
    int cnt = 0;
    // 3 leaves?
    REP(j, 0, l) {
      int w = ch[i][j];
      if (ch[w].size() == 0) {
	cnt += 1;
      }
    }
    if (cnt < 3) {
      cout << "No\n";
      return 0;
    }
  }
  cout << "Yes\n";
}
