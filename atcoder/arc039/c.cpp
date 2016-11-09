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

struct cell {
  cell *link[4];
  PI coord;
  PI lc[4];
};

const int N = 1000000;
cell pool[N];
int pos;

map<PI, cell*> tbl;

cell *get(PI c) {
  if (tbl.count(c)) {
    return tbl[c];
  }
  cell *n = &pool[pos++];
  REP(i, 0, 4) {
    n->link[i] = NULL;
  }
  n->coord = c;
  tbl[c] = n;
  return n;
}

void debug(void) {
  REP(i, 0, pos) {
    cerr << "[" << i << "]";
    cerr << "(" << pool[i].coord.first << "," << pool[i].coord.second << ") ";
    REP(j, 0, 4) {
      if (pool[i].link[j]) {
	int idx = pool[i].link[j] - pool;
	cerr << "[" << idx << "]";
	//cerr << "(" << pool[i].lc[j].first << "," << pool[i].lc[j].second << ")";
      } else {
	cerr << "[ ]";
      }
    }
    cerr << endl;
  }
}

const int DEBUG = 0;

int main(void){
  int k;
  string s;
  cin >> k >> s;
  cell *cur = get(PI(0, 0));
  int dx[4] = {1, 0, -1, 0};
  int dy[4] = {0, 1, 0, -1};
  REP(i, 0, 4) {
    cell *tmp = get(PI(dx[i], dy[i]));
    cur->link[i] = tmp;
  }
  REP(i, 0, 4) {
    cur->link[i]->link[(i+2)%4] = cur->link[(i+2)%4];
    cur->link[i]->lc[(i+2)%4] = cur->link[(i+2)%4]->coord;
    cur->lc[i] = cur->link[i]->coord;
  }
  REP(loop_cnt, 0, k) {
    if (DEBUG) {
      cerr << "loop[" << loop_cnt << "]" << endl;
      cerr << "current = " << cur - pool << endl;
      debug();
    }
    int dir = -1;
    switch (s[loop_cnt]) {
    case 'R':
      dir = 0;
      break;
    case 'U':
      dir = 1;
      break;
    case 'L':
      dir = 2;
      break;
    case 'D':
      dir = 3;
      break;
    default:
      {}
    }
    cell *before = cur;
    cur = cur->link[dir];
    REP(i, 0, 4) {
      if (cur->link[i] == NULL) {
	cur->link[i] = get(PI(cur->coord.first + dx[i], cur->coord.second + dy[i]));
      }
    }
    REP(i, 0, 4) {
      cur->link[i]->link[(i+2)%4] = cur->link[(i+2)%4];
    }
  }
  cout << cur->coord.first << " " << cur->coord.second << endl;
}
