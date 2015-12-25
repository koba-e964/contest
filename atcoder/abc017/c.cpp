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
typedef pair<int, int> PI;
const double EPS=1e-9;
const int N = 100010;
int n,m;


int qq[N];

int main(void){
  cin >> n >> m;
  int tot = 0;
  
  REP(i, 0, n) {
    int l, r, s;
    cin >> l >> r >> s;
    l--, r--;
    tot += s;
    qq[l] += s;
    qq[r + 1] -= s;
  }
  int mi = 1e9;
  REP(i, 1, m) {
    qq[i] += qq[i - 1];
  }
  REP(i, 0, m) {
    mi = min(mi, qq[i]);
  }
  cout << tot - mi << endl;
}
