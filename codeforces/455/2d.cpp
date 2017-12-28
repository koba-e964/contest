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
  string s;
  cin >> s;
  int n = s.length();
  vector<pair<char, int> > occ;
  int cur = 0;
  REP(i, 0, n) {
    if (i == 0 || s[i - 1] != s[i]) {
      if (cur > 0) {
	occ.push_back(make_pair(s[i - 1], cur));
      }
      cur = 1;
    } else {
      cur += 1;
    }
  }
  if (cur > 0) {
    occ.push_back(make_pair(s[n - 1], cur));
    cur = 0;
  }
  int ans = 0;
  while (occ.size() > 1) {
    vector<pair<char, int> > new_occ;
    int turn = 1e8;
    REP(i, 0, occ.size()) {
      if (i == 0 || i == (int) occ.size() - 1) {
	turn = min(turn, occ[i].second);
      } else {
	turn = min(turn, (occ[i].second + 1) / 2);
      }
    }
    REP(i, 0, occ.size()) {
      pair<char, int> nc('$', 0);
      if (i == 0 || i == (int) occ.size() - 1) {
	if (occ[i].second > turn) {
	  nc = make_pair(occ[i].first, occ[i].second - turn);
	}
      } else {
	if (occ[i].second > 2 * turn) {
	  nc = make_pair(occ[i].first, occ[i].second - 2 * turn);
	}
      }
      if (nc.first != '$') {
	if (new_occ.size() > 0 && new_occ.back().first == nc.first) {
	  new_occ.back().second += nc.second;
	} else {
	  new_occ.push_back(nc);
	}
      }
    }
    occ = new_occ;
    ans += turn;
    if (0) {
      cerr << "occ:";
      REP(i, 0, occ.size()) {
	cerr << " " << occ[i].first << "," << occ[i].second;
      }
      cerr << endl;
    }
  }
  cout << ans << "\n";
}
