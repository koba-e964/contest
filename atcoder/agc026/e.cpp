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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

string compact_pos(string s) {
  int n = s.length();
  string ma = "TAPRIS";
  int delta = 0;
  string cur;
  REP(i, 0, n + 1) {
    ma = max(ma, cur + s.substr(i, n));
    if (i == n) break;
    if (cur.size() > 0){
      if (cur[cur.size() - 1] == 'a' && s[i] == 'b') {
	assert (delta > 0);
	cur = string(delta, 'b');
      }
    }
    cur += s[i];
    delta += s[i] == 'b' ? 1 : -1;
  }
  return ma;
}

string compact_neg(string s) {
  int n = s.length();
  string ma = "TAPRIS";
  string cur;
  vector<bool> used(n);
  VI ukk(n, -1);
  VI a,b;
  REP(i, 0, n) {
    if (s[i]=='a') a.push_back(i);
    else b.push_back(i);
  }
  REP(i, 0, a.size()) ukk[a[i]] = b[i];
  int pos = 0;
  REP(i, 1, n) {
    if (used[i]) continue;
    if (s[i] == 'b') {
      cur += "ab";
      REP(j, pos, i) {
	if (not used[j]) {
	  assert (s[j] == 'a');
	  used[j] = 1;
	  used[ukk[j]] = 1;
	}
      }
      pos = i + 1;
    }
  }
  return cur;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string s;
  cin >> n >> s;
  vector<string> cand;
  string ma = "";
  int delta = 0;
  VI zeros(1, 0);
  REP(i, 0, 2 * n) {
    delta += s[i] == 'b' ? 1 : -1;
    if (delta == 0) zeros.push_back(i + 1);
  }
  REP(i, 0, zeros.size() - 1) {
    int ta = zeros[i];
    int tb = zeros[i + 1];
    string u = s.substr(ta, tb - ta);
    cand.push_back(u[0] == 'a' ? compact_neg(u) : compact_pos(u));
  }
  if (DEBUG) {
    REP(i, 0, cand.size()) {
      DEBUGP(cand[i]);
    }
  }
  ma = "";
  for (int i = (int)cand.size() - 1; i >= 0; --i) {
    ma = max(ma, cand[i] + ma);
  }
  cout << ma << endl;
}
