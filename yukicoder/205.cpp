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
  vector<string> s(n);
  REP(i, 0, n) {
    cin >> s[i];
  }
  priority_queue<string, vector<string>, greater<string> > que;
  REP(i, 0, n) {
    que.push(s[i] + "}");
  }
  string t;
  while (not que.empty()) {
    string a = que.top(); que.pop();
    if (a == "}") { continue; }
    t += a[0];
    que.push(a.substr(1));
  }
  cout << t << endl;
}
