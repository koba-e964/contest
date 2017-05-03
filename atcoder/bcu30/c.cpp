#include <iostream>
#include <algorithm>
#include <vector>
#include <cmath>
#include <map>
#include <set>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;
 
int main(void) {
int n, m;
cin>>n>>m;
vector<vector<int> > mat(n, vector<int>(n));
rep(i, m) {
int a, b;
cin >> a >> b;
a--, b--;
mat[a][b] = 1;
}
vector<vector<int> > mat2(n, vector<int>(n));
rep(k, n)rep(i,n)rep(j,n) {
mat2[i][j] += mat[i][k] * mat[k][j];
}
i64 tot = 0;
rep(i, n)rep(j, n) {
tot += (i64)mat2[i][j] * mat2[i][j];
}
cout << tot << endl;
}
