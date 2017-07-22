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
typedef vector<int> VI;
typedef pair<int, int> PI;
typedef pair<int, PI> PIPI;

const int DIE = 43200;

string convert(int time) {
  int h = time / 3600;
  int m = time / 60 % 60;
  int s = time % 60;
  stringstream ss;
  ss << setfill('0') << setw(2) << h
     << ":" << setfill('0') << setw(2) << m
     << ":" << setfill('0') << setw(2) << s;
  return ss.str();
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  while (cin >> n && n) {
    vector<vector<bool> > tbl(DIE, vector<bool>(n, false));
    REP(i, 0, n) {
      int u[3];
      cin >> u[0] >> u[1] >> u[2];
      set<int> times;
      int perm[3] = {0, 1, 2};
      do {
	REP(r, 0, 60) {
	  int h = u[perm[0]];
	  int m = u[perm[1]];
	  int s = u[perm[2]];
	  h = (h + r) % 60;
	  m = (m + r) % 60;
	  s = (s + r) % 60;
	  // Check if they are consistent
	  if (h % 5 != m / 12) { continue; }
	  int time = (h / 5) * 3600 + m * 60 + s;
	  times.insert(time);
	}
      } while(next_permutation(perm, perm + 3));
      for (set<int>::iterator it = times.begin(); it != times.end(); ++it) {
	tbl[*it][i] = true;
      }
    }
    // Shakutori
    int pos = 0;
    PIPI mi(1e8, PI(0, 0));
    int kind = 0;
    VI freq(n, 0);
    REP(i, 0, DIE) {
      while (pos < DIE && kind < n) {
        REP(j, 0, n) {
	  if (tbl[pos][j]) {
	    freq[j] += 1;
	    if (freq[j] == 1) {
	      kind += 1;
	    }
	  }
	}
	pos += 1;
      }
      if (kind >= n) {
	mi = min(mi, PIPI(pos - i, PI(i, pos - 1))); // note: inclusive!
      }
      // decrement
      REP(j, 0, n) {
	if (tbl[i][j]) {
	  freq[j] -= 1;
	  if (freq[j] == 0) { kind -= 1; }
	}
      }
    }
    cout << convert(mi.second.first) << " " << convert(mi.second.second) << "\n";
  }
}
