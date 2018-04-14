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

bool check(const vector<string> &s, const VI &p) {
  int h = s.size();
  int w = p.size();
  vector<string> t(h, string(w, '+'));
  vector<string> tr(h, string(w, '+'));
  REP(i, 0, h) {
    REP(j, 0, w) {
      t[i][j] = s[i][p[j]];
    }
    tr[i] = t[i];
    reverse(tr[i].begin(), tr[i].end());
  }
  if (DEBUG) {
    cerr << "t:";
    REP(i, 0, h) cerr << t[i] << endl;
  }
  vector<bool> used(h, false);
  REP(i, 0, h) {
    if (used[i]) continue;
    bool found = false;
    REP(j, i + 1, h) {
      if (used[i]) continue;
      string r = tr[j];
      if (t[i] == r) {
	used[j] = true;
	used[i] = true;
	found = true;
        break;
      }
    }
  }
  int rem = 0;
  int idx = -1;
  REP(i, 0, h) {
    if (not used[i]) {
      rem++;
      idx = i;
    }
  }
  if (rem != h % 2) return false;
  if (rem == 0) return true;
  string r = tr[idx];
  return t[idx] == r;
}

bool dfs(int v, const vector<string> &s, VI &p, bool centre, int used, vector<bool> &vis) {
  int w = s[0].length();
  if (vis[v]) {
    return dfs(v + 1, s, p, centre, used, vis);
  }
  if (v >= w) {
    if (DEBUG){
      REP(i, 0, w) {
	cerr << " " << p[i];
      }
      cerr << endl;
    }
    return check(s, p);
  }
  if (w % 2 == 1 && not centre) {
    p[w / 2] = v;
    vis[v] = 1;
    if (dfs(v + 1, s, p, true, used, vis)) return true;
    p[w / 2] = -1;
    vis[v] = 0;
  }
  if (used < w / 2) {
    REP(i, v + 1, w) {
      if (vis[i]) continue;
      p[used] = v;
      p[w - 1 - used] = i;
      vis[v] = 1;
      vis[i] = 1;
      if (dfs(v + 1, s, p, centre, used + 1, vis)) return true;
      p[used] = -1;
      p[w - 1 - used] = -1;
      vis[v] = 0;
      vis[i] = 0;
    }
  }
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
  REP(i, 0, h) cin >> s[i];
  VI p(w);
  REP(i, 0, w) p[i] = -1;
  vector<bool> vis(w);
  bool ok = dfs(0, s, p, false, 0, vis);
  cout << (ok ? "YES" : "NO") << endl;
}
