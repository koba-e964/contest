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

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  mt19937_64 mt(time(0));
  int n, m;
  cin >> n >> m;
  VL s(n);
  vector<VI> p(n);
  vector<vector<PI>> solve_idx(m);
  VL tot(m);
  VL st(m);
  VL rand(n);
  REP(i, 0, n) {
    rand[i] = mt();
  }
  // 支配関係。dom[i][j] == true だったら i は j に勝ったことがある
  vector<vector<bool>> dom(m, vector<bool>(m));
  REP(i, 0, n) {
    cin >> s[i];
    int k;
    cin >> k;
    p[i] = VI(k);
    REP(j, 0, k) {
      cin >> p[i][j];
      p[i][j]--;
      solve_idx[p[i][j]].push_back(PI(i, j));
      tot[p[i][j]] += s[i];
      st[p[i][j]] ^= rand[i];
    }
    // O(K^2)、合計 O(NM)
    REP(j, 0, k) {
      REP(l, 0, j) {
        dom[p[i][l]][p[i][j]] = true;
      }
    }
  }
  REP(i, 0, m) assert(!dom[i][i]);
  // 最小値は常に他のチームが時間ギリギリで解いたことにすれば良いので楽。
  // 最大値が難しい。相手チームごとに自分の解いた問題のどれが遅かったかを計算して都合よく合わせることができないため。
  // どの問題を最後に解いたか決め打ちして、その問題のそれ以前および他の問題は開始 0 秒で解かれたことにすると良い。
  REP(i, 0, m) {
    int loss = 0;
    set<int> possible_loss;
    REP(j, 0, m) {
      if (tot[i] < tot[j] || (tot[i] == tot[j] && st[i] == st[j] && dom[j][i] && !dom[i][j])) {
        loss++;
        continue;
      }
      if (tot[i] == tot[j] && (st[i] != st[j] || (dom[j][i] && dom[i][j]))) {
        possible_loss.insert(j);
      }
    }
    int ma = 0;
    for (PI idx: solve_idx[i]) {
      int prob = idx.first;
      int rank = idx.second;
      int cnt = 0;
      REP(j, rank + 1, p[prob].size()) {
        if (possible_loss.count(p[prob][j])) {
          cnt++;
        }
      }
      ma = max(ma, (int) possible_loss.size() - cnt);
    }
    cout << loss + 1 << " " << loss + ma + 1 << "\n";
  }
}
