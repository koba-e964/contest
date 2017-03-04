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
typedef pair<int, PI> PIPI;

const int H = 50;

const int PC = 600; // the number of sol.
const int BC = 5; // branch

// Takes H strings of length H
vector<PI> solve(vector<string> s) {
  vector<PI> res;
  vector<vector<bool> > used(H, vector<bool>(H, true));
  int nonzero = 0;
  REP(i, 0, H) {
    REP(j, 0, H) {
      if (s[i][j] != '0') {
	nonzero += 1;
	used[i][j] = false;
      }
    }
  }
  while (nonzero > 0) {
    // Find the max element among them
    int ma = 0;
    int maxx = -1, maxy = -1;
    REP(i, 0, H) {
      REP(j, 0, H) {
	int c = s[i][j] - '0';
	if (not used[i][j] && ma < c) {
	  ma = c;
	  maxx = i, maxy = j;
	}
      }
    }
    // Find 8 pieces (path only, tentative)
    queue<vector<PI> > que;
    que.push(vector<PI>(1, PI(maxx, maxy)));
    vector<PI> obtained;
    int masc = 0;
    int pickcnt = 0;
    while (not que.empty()) {
      vector<PI> path = que.front(); que.pop();
      if (path.size() >= 8) {
	if (pickcnt >= PC) { break; }
	int score = 1;
	REP(i, 0, 8) {
	  PI t = path[i];
	  score *= s[t.first][t.second] - '0';
	}
	if (masc < score) {
	  obtained = path;
	  masc = score;
	}
	pickcnt += 1;
	continue;
      }
      int dx[4] = {1, 0, -1, 0};
      int dy[4] = {0, 1, 0, -1};
      vector<PIPI> adj;
      REP(i, 0, path.size()) {
	PI t = path[i];
	
	REP(d, 0, 4) {
	  int nx = t.first + dx[d];
	  int ny = t.second + dy[d];
	  if (nx < 0 || nx >= H || ny < 0 || ny >= H) { continue; }
	  if (find(path.begin(), path.end(), PI(nx, ny)) != path.end())
	    { continue; }
	  int sc = s[nx][ny] - '0';
	  if (sc == 0) { continue; }
	  adj.push_back(PIPI(sc, PI(nx, ny)));
	}
      }
      sort(adj.rbegin(), adj.rend()); // adjacent cell with max number
      path.push_back(PI(-1, -1));
      int lim = min((size_t)BC, adj.size());
      if (path.size() == 7) { lim = min(lim, 1); }
      REP(i, 0, lim) {
	PI nxt = adj[i].second;
	path[path.size() - 1] = nxt;
	que.push(path);
      }
    }
    if (obtained.size() < 8) {
      // disable (maxx, maxy)
      used[maxx][maxy] = true;
      nonzero -= 1;
      continue;
    }
    // Use these eight cells
    REP(i, 0, 8) {
      PI t = obtained[i];
      s[t.first][t.second] = '0';
      if (used[t.first][t.second] == false) {
	nonzero -= 1;
      }
      used[t.first][t.second] = true;
      res.push_back(t);
    }
  }
  return res;
}


int main(void){
  int h, w, k;
  cin >> h >> w >> k;
  // k == 8
  vector<string> s(H);
  REP(i, 0, H) {
    cin >> s[i];
  }
  vector<PI> result = solve(s);
  cout << result.size() / k << endl;
  REP(i, 0, result.size()) {
    PI t = result[i];
    cout << t.first + 1 << " " << t.second + 1 << endl;
  }
}
