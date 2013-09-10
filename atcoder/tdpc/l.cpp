#include <vector>
#include <string>
#include <algorithm>
#include <iostream>
#include <cstdio>
#include <cassert>
#include <cstdlib>

using namespace std;
const int N=1000;
int f[N][N];
int n;
int main(void){
	cin>>n;
	for(int i=0;i<n;i++){
		for(int j=0;j<n;j++){
			cin>>f[i][j];
		}
	}
}