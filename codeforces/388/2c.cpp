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



int main(void){
  int n;
  cin >> n;
  string s;
  cin >> s;
  int r[2] = {0, 0};
  REP(i, 0, n) {
    if (s[i] == 'D') {
      r[0]++;
    } else {
      r[1]++;
    }
  }
  queue<int> que;
  REP(i, 0, n) {
    que.push(s[i] == 'D' ? 0 : 1);
  }
  int qr[2] = {0, 0};
  while (r[0] != 0 && r[1] != 0) {
    int x = que.front(); que.pop();
    if (qr[x] > 0) {
      qr[x] --;
      r[x]--;
    } else {
      que.push(x);
      qr[1 - x]++;
    }
  }
  cout << (r[0] == 0 ? "R" : "D") << endl;
}
