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

int dist(int x, int y) {
  return abs(x / 3 - y / 3) + abs(x % 3 - y % 3);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string n;
  cin >> n;
  int ans = n.size() * 4;
  VI p(9);
  REP(i, 0, 9) p[i] = i;
  vector<VI> freq(9, VI(9, 0));
  REP(i, 0, n.size() - 1) {
    int x = n[i] - '1';
    int y = n[i + 1] - '1';
    freq[x][y] += 1;
  }
  do {
    int sum = 0;
    REP(i, 0, 9) {
      REP(j, 0, 9) sum += dist(p[i], p[j]) * freq[i][j];
    }
    sum += dist(p[n[0] - '1'], 0);
    ans = min(ans, sum);
  } while (next_permutation(p.begin(), p.end()));
  cout << ans + n.size() << endl;
}
