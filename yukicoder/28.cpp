#include <algorithm>
#include <iostream>
#include <map>
#include <queue>
#include <vector>

void factorize(long long v, std::map<long long, int> &result) {
  long long p = 2;
  while (v > 1 || p * p <= v) {
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    if (cnt > 0) {
      if (result.count(p) == 0) {
	result[p] = 0;
      }
      result[p] += cnt;
    }
    p += p == 2 ? 1 : 2;
  }
}

std::pair<std::vector<int>, long long>
factor_base(long long v, const std::vector<long long> &base) {
  int n = base.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    long long p = base[i];
    int cnt = 0;
    while (v % p == 0) {
      cnt++;
      v /= p;
    }
    ret[i] = cnt;
  }
  return std::pair<std::vector<int>, long long>(ret, v);
}

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
const ll mod = 1e8 + 9;


// Solved after reading the editorial
int solve(const vector<VI> &tbl, const VI &bias, int k) {
  int n = tbl.size();
  int m = bias.size();
  int mi = 1e8;
  REP(i, 0, m) {
    priority_queue<int, VI, less<int> > que;
    REP(j, 0, n) {
      que.push(tbl[j][i]);
      if (que.size() > k) {
	que.pop();
      }
    }
    int tot = 0;
    REP(j, 0, k) {
      tot += que.top();
      que.pop();
    }
    mi = min(mi, tot / bias[i]);
  }
  return mi;
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> q;
  while (q--) {
    ll seed;
    int n, k, b;
    cin >> seed >> n >> k >> b;
    ll cur = seed;
    map<int, int> freq;
    map<ll, int> b_fact;
    factorize(b, b_fact);
    vector<pair<ll, int> > b_fact_v(b_fact.begin(), b_fact.end());
    vector<ll> fbase;
    VI bias;
    REP(i, 0, b_fact_v.size()) {
      fbase.push_back(b_fact_v[i].first);
      bias.push_back(b_fact_v[i].second);
    }
    vector<VI> tbl(n + 1);
    REP(i, 0, n + 1) {
      VI v = factor_base(cur, fbase).first;
      tbl[i] = v;
      cur = 1 + (cur * (cur + 12345) % mod);
    }
    cout << solve(tbl, bias, k) << "\n";
  }
}
