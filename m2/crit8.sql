SELECT AbilityI, COUNT(*) 
    FROM Pokemon 
    GROUP BY AbilityI
    HAVING Color = 'Red';
    
SELECT Color FROM Pokemon;

SELECT AVG(IndepYear) FROM Country;
SELECT Region, MIN(SurfaceArea), MAX(SurfaceArea) FROM Country;
SELECT SUM(SurfaceArea) FROM Country;