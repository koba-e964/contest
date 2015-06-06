#include <algorithm>
#include <iostream>
#include <string>
#include <cassert>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
const int DEBUG = 0;

int h, w;
string s[51];
bool bl(int x, int y) {
  if (x < 0 || x >= h || y < 0 || y >= w) {
    return false;
  }
  return s[x][y] == '#';
}

int nxt(int s, bool v) {
  if (s == 0) {
    return (int)v;
  }
  if (s == 1) {
    return v ? 0 : 2;
  }
  return 2;
}

#define  UPD 	  cnt = nxt(cnt, bl(i + dx * k, j + dy * k))
bool solve(int dx, int dy) {
  if (dx == 0) {
    REP(i, 0, h) {
      REP(j, 0, dy) {
	int cnt = 0;
	REP(k, 0, (w + dy - 1) / dy) {
	  UPD;
	}
	if (cnt) {
	  return false;
	}
      }
    }
    return true;
  }
  assert(dx > 0);
  if(dy >= 0) {
    REP(j, - dy * (h - 1) / dx - 1, w) {
      REP(i, 0, dx) {
	int cnt = 0;
	REP(k, 0, h / dx + 1) {
	  UPD;
	}
	if (cnt) {
	  return false;
	}
      }
    }
    return true;
  }
  assert(dy < 0);
  REP(j, 0, w - dy * (h - 1) / dx + 1) {
    REP(i, 0, dx) {
      int cnt = 0;
      REP(k, 0, h / dx + 1) {
	UPD;
      }
      if (cnt) {
	return false;
      }
    }
  }
  return true;
}

int main(void){
  cin >> h >> w;
  REP (i, 0, h) {
    cin >> s[i];
  }
  int bl = 0;
  REP(i, 0, h) {
    bl += count(s[i].begin(), s[i].end(), '#');
  }
  if (bl == 0) {
    cout << "NO" << endl;
    return 0;
  }
  REP(dx, 1, h) {
    REP(dy, -w + 1, w) {
      if (solve(dx, dy)) {
	if (DEBUG) {
	  cout << "dx=" << dx << ", dy=" << dy << endl;
	}
	cout << "YES" << endl;
	return 0;
      }
    }
  }
  REP(dy, 1, w) {
    if (solve(0, dy)) {
      if (DEBUG) {
	cout << "dx=0, dy=" << dy << endl;
      }
      cout << "YES" << endl;
      return 0;
    }
  }
  cout << "NO" << endl;
}
