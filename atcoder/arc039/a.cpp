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

int solve(int c[3], int d[3], int s, int i) {
  int cc[3] = {c[0],c[1],c[2]};
  int dd[3] = {d[0],d[1],d[2]};
  if (s) {
    dd[i] = i == 0 ? 1 : 0;
  } else {
    cc[i] = 9;
  }
  return 100*(cc[0]-dd[0]) + 10*(cc[1]-dd[1]) + cc[2]-dd[2];
}

int main(void){
  int a,b;
  cin >> a >> b;
  int c[3] = {a/100, (a/10)%10, a%10};
  int d[3] = {b/100,b/10%10, b%10};
  int m= -1000000;
  REP(i,0,3){
    m = max(m, solve(c,d,0,i));
    m = max(m, solve(c,d,1,i));
  }
  cout << m << endl;
}
