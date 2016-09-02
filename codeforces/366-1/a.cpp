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

const int N = 300100;
bool mark[N];
queue<int> e[N];
queue<PI> que;

int main(void){
  int n, q;
  scanf("%d%d", &n, &q);
  int cnt = 0;
  int ans = 0;
  REP(i, 0, q) {
    int ty, x;
    scanf("%d%d", &ty, &x);
    switch(ty) {
    case 1:
      e[x].push(cnt);
      que.push(PI(x, cnt));
      cnt++;
      ans++;
      break;
    case 2:
      while (!e[x].empty()) {
	int p = e[x].front(); e[x].pop();
	if (! mark[p]) {
	  ans--;
	}
	mark[p] = 1;
      }
      break;
    case 3:
      while (!que.empty() && que.front().second < x) {
	PI p = que.front(); que.pop();
	int i = p.first;
	int t = p.second;
	if (! mark[t]) {
	  mark[t] = 1;
	  ans--;
	}
      }
    }
    printf("%d\n", ans);
  }
}
