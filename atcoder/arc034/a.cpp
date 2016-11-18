#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>
 
using namespace std;
 
typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define rep(i,s,n) for(int i=(s); i < int(n); ++i)
 
const int N = 1e6;
int a[N];
int n;
 
int main(void) {
  cin >> n;
  int m=0;
  rep(i,0,n) {
    int a,b,c,d,e;
    cin>>a>>b>>c>>d>>e;
    m = max(m, 90*(a+b+c+d) + 11*e);
  }
  printf("%f\n", (m / 90.0));
}

