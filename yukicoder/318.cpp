#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void){
  int n;
  cin >> n;
  VL a(n);
  set<ll> a_s;
  REP(i, 0, n) {
    cin >> a[i];
    a_s.insert(a[i]);
  }
  VL comp(a_s.begin(), a_s.end());
  map<ll, int> inv_comp;
  int m = comp.size();
  REP(i, 0, m) {
    inv_comp[comp[i]] = i;
  }
  vector<PI> range(m, PI(n, 0));
  REP(i, 0, n) {
    ll v = a[i];
    int idx = inv_comp[v];
    range[idx].first = min(range[idx].first, i);
    range[idx].second = max(range[idx].second, i);
  }
  vector<PI> actions;
  REP(i, 0, m) {
    actions.push_back(PI(range[i].first, 2 * i + 0));
    actions.push_back(PI(range[i].second, 2 * i + 1));
  }
  sort(actions.begin(), actions.end());
  VI result(n);
  set<int> active;
  int cur = -1;
  REP(i, 0, 2 * m) {
    PI cur2 = actions[i];
    int t = cur2.first;
    int idx = cur2.second / 2;
    if (active.size() > 0) {
      int midx = *active.rbegin();
      while (cur < t) {
	cur++;
	result[cur] = comp[midx];
      }
    }
    if (cur2.second % 2 == 0) {
      active.insert(idx);
      int midx = *active.rbegin();
      result[t] = comp[midx];
    }
    if (cur2.second % 2 == 1) {
      assert (active.count(idx));
      active.erase(idx);
    }
  }
  REP(i, 0, n) {
    cout << result[i] << (i == n - 1 ? "\n" : " ");
  }
}
