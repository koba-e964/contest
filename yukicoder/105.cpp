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
const double EPS=1e-10;



int main(){
int t;cin>>t;
for(int i=0;i<t;i++){
double mi = 10000;
for(int j =0;j<6;j++){
double x,y;cin>>x>>y;
double th = atan2(y, x);
if (th > - EPS) mi = min(mi, th);
}
printf("%.9f\n",  mi * 180.0 / acos(-1));
}
}
