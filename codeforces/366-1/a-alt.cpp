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

int uniq = 0;
int unread = 0;
const int N = 323456;
int rt[N];
VI pos[N];
int cur[N];
int vis[N];
int first_memo;

int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  int n, q;
  cin >> n >> q;
  REP(loop_cnt, 0, q) {
    int ty, x;
    cin >> ty >> x;
    if (ty == 1) {
      rt[uniq] = x;
      pos[x].push_back(uniq);
      vis[uniq] = 1;
      unread += 1;
      uniq++;
    } else if (ty == 2) {
      while (cur[x] < pos[x].size()) {
	int &old = vis[pos[x][cur[x]]];
	if (old) {
	  unread -= 1;
	}
	old = 0;
	cur[x] += 1;
      }
    } else {
      REP(i, first_memo, x) {
	int &old = vis[i];
	if (old) {
	  unread -= 1;
	}
	old = 0;
      }
      first_memo = max(first_memo, x);
    }
    cout << unread << "\n";
  }
}
