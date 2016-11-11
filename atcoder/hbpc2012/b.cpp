#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


map<string, int> memo;
int get(const string &s) {
  if (not memo.count(s)) {
    int len = memo.size();
    memo[s] = len;
    return len;
  }
  return memo[s];
}

int to_time(const string &s) {
  stringstream is(s);
  int h, m;
  is >> h;
  is.ignore();
  is >> m;
  int res = h * 60 + m;
  return res;
}

void dfs(int v, int t, const vector<vector<PI> > &cons, VI &times) {
  assert (t >= 0 && t <= 1439);
  if (times[v] >= 0) {
    return;
  }
  times[v] = t;
  REP(i, 0, cons[v].size()) {
    int w = cons[v][i].first;
    int diff = cons[v][i].second;
    dfs(w, (t + diff) % 1440, cons, times);
  }
}

int main(void){
  string city_a;
  cin >> city_a;
  vector<string> hour(24);
  REP(i, 0, 24) {
    cin >> hour[i];
  }
  int n;
  cin >> n;
  vector<vector<PI> > cons;
  REP(i, 0, n) {
    string c1, t1, c2, t2;
    cin >> c1 >> t1 >> c2 >> t2;
    int ci1, ti1, ci2, ti2;
    ci1 = get(c1);
    ci2 = get(c2);
    ti1 = to_time(t1);
    ti2 = to_time(t2);
    if (max(ci1, ci2) >= cons.size()) {
      cons.push_back(vector<PI>());
      cons.push_back(vector<PI>());
    }
    cons[ci1].push_back(PI(ci2, (ti2 - ti1 + 1440) % 1440));
    cons[ci2].push_back(PI(ci1, (ti1 - ti2 + 1440) % 1440));
  }
  string city_B, time_B;
  cin >> city_B >> time_B;
  int hour_idx;
  if (city_a == city_B) {
    hour_idx = to_time(time_B) / 60;
  } else {
    VI times(memo.size(), -1);
    assert (cons.size() >= memo.size());
    dfs(get(city_B), to_time(time_B), cons, times);
    hour_idx = times[get(city_a)] / 60;
  }
  cout << hour[hour_idx] << endl;
}
