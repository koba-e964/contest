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

const int H = 30;
string s[H];
int h, w, k;

const int DEBUG = 0;

ll calc(int DEBUG) {
  string t[H];
  REP(i, 0, h) {
    t[i] = s[i];
  }
  ll sum = 0;
  int turn = 0;
  while (1) {
    // fall
    bool fell = 0;
    REP(i, 0, w) {
      int pos = h - 1;
      for (int j = h - 1; j >= 0; --j) {
	if (t[j][i] != '.') {
	  if (pos != j) {
	    swap(t[pos][i], t[j][i]);
	    fell = 1;
	  }
	  pos--;
	}
      }
    }
    if (DEBUG) {
      cout << "turn " << turn << endl;
      REP(i, 0, h) {
	cout << t[i] << endl;
      }
    }
    if (not fell) {
      break;
    }
    // perish
    REP(i, 0, h) {
      int cnt = 0;
      REP(j, 0, w + 1) {
	if ((j > 0 && j < w && t[i][j] == t[i][j - 1])) {
	  cnt++;
	} else {
	  if (cnt >= k) {
	    //perish
	    ll sc = t[i][j - 1] - '0';
	    if (t[i][j - 1] == '.') {
	      sc = 0;
	    }
	    sc = sc << turn;
	    sum += sc * cnt;
	    REP(l, 0, cnt) {
	      t[i][j - cnt + l] = '.';
	    }
	  }
	  cnt = 0;
	  if (j < w) {
	    cnt++;
	  }
	}
      }
    }
    turn++;
  }
  return sum;
}

int main(void){
  cin >> h >> w >> k;
  REP(i, 0, h) {
    cin >> s[i];
  }

  ll ma = 0;
  REP(i, 0, h) {
    REP(j, 0, w) {
      char tmp = s[i][j];
      s[i][j] = '.';
      ma = max(ma, calc(i == 3 && j == 2 && DEBUG));
      s[i][j] = tmp;
    }
  }
  cout << ma << endl;
}
