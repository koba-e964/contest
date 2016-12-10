#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<ll, ll> PL;

pair<PL, pair<int, PL> > solve(const VI &p) {
  int n = p.size();
  int centre_idx = -1;
  REP(j, 0, n - 1) {
    if (p[j] >= 0 && p[j + 1] < 0) {
      centre_idx = j;
    }
  }
  assert (centre_idx >= 0);
  ll o = 0;
  ll x = 0;
  REP(i, 0, centre_idx + 1) {
    assert (p[i] >= 0);
    o += p[centre_idx] - p[i] - i;
  }
  REP(i, centre_idx + 1, n) {
    assert (p[i] < 0);
    x += - p[i] + p[centre_idx + 1] - (i - centre_idx - 1);
  }
  ll dist = - p[centre_idx + 1] - p[centre_idx];
  return make_pair(PL(o, x), pair<int, PL>(dist, PL(centre_idx + 1, n - centre_idx - 1)));
}

int main(void){
  int h, w;
  cin >> h >> w;
  const int inf = 1e5;
  int o_mi = inf;
  int x_mi = inf;
  vector<VI> pool;
  REP(idiom, 0, h) {
    string s;
    cin >> s;
    assert (s.length() == w);
    char last = '\0';
    int last_pos = -1;
    VI chunk;
    REP(j, 0, w) {
      if (s[j] == 'o') {
	if (last == 'x' && chunk.size() > 0) {
	  pool.push_back(chunk);
	  chunk.clear();
	}
	chunk.push_back(j);
	last = 'o';
	last_pos = j;
      }
      if (s[j] == 'x') {
	if (last == '\0') {
	  x_mi = min(x_mi, j);
	}
	chunk.push_back(-j);
	last = 'x';
	last_pos = j;
      }
    }
    if (last == 'o') {
      o_mi = min(o_mi, w - 1 - last_pos);
    } else if (chunk.size() > 0) {
      pool.push_back(chunk);
    }
  }
  if (o_mi != inf || x_mi != inf) {
    cout << (o_mi <= x_mi ? 'o' : 'x') << endl;
    return 0;
  }
  ll o = 0;
  ll x = 0;
  vector<pair<int, PL> > result(pool.size());
  vector<PL> sorter;
  REP(i, 0, pool.size()) {
    pair<PL, pair<int, PL> > res = solve(pool[i]);
    result[i] = res.second;
    sorter.push_back(PL(result[i].second.first + result[i].second.second,
			i));
    if (0) {
      cerr << "solve([";
      REP(j, 0, pool[i].size()) {
	cerr << pool[i][j] << " ";
      }
      cerr << "])=(" << res.first.first << "," << res.first.second
		  << ")," << res.second.first
		  << ",(" << res.second.second.first << "," << res.second.second.second << ")" << endl;
    }
    o += res.first.first;
    x += res.first.second;
  }
  sort(sorter.rbegin(), sorter.rend());
  int turn = 0;
  REP(i, 0, sorter.size()) {
    int idx = sorter[i].second;
    while (result[idx].first >= 3) {
      result[idx].first--;
      if (turn == 0) {
	o += result[idx].second.first;
      } else {
	x += result[idx].second.second;
      }
      turn = 1 - turn;
    }
  }
  cout << (o <= x ? 'x' : 'o') << endl;
}
